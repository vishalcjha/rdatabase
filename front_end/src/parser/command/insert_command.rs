use nom::{
    bytes::complete::{tag, tag_no_case},
    character::complete::{alphanumeric1, space0, space1},
    combinator::opt,
    multi::separated_list1,
    sequence::{delimited, tuple},
};

use crate::parser::NomParsable;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    NamedValue(String, String),
    UnnamedValue(String),
}

impl Value {
    #[allow(dead_code)]
    fn new_named_value(name: impl Into<String>, value: impl Into<String>) -> Value {
        Value::NamedValue(name.into(), value.into())
    }

    #[allow(dead_code)]
    fn new_unnamed_value(value: impl Into<String>) -> Value {
        Value::UnnamedValue(value.into())
    }
}

// simple parser to parse "(a, b, c) and return vec!["a", "b", "c"]
fn wrapped_parser(input: &str) -> nom::IResult<&str, Vec<&str>> {
    delimited(
        tag("("),
        separated_list1(tag(","), tuple((space0, alphanumeric1))),
        tag(")"),
    )(input.trim())
    // remove the matched space
    .map(|res| (res.0, res.1.into_iter().map(|it| it.1).collect()))
}

fn parse_value_list(input: &str) -> nom::IResult<&str, Vec<Value>> {
    let just_values = opt(tuple((tag_no_case("values"), space0, wrapped_parser)))(input.trim())?;
    match just_values {
        (left, Some(values)) => Ok((
            left,
            values
                .2
                .iter()
                .map(|v| Value::UnnamedValue(String::from(*v)))
                .collect::<Vec<_>>(),
        )),
        (left, None) => {
            let (left, (names, _, _, _, values)) = tuple((
                wrapped_parser,
                space1,
                tag_no_case("values"),
                space0,
                wrapped_parser,
            ))(left.trim())?;
            let values = names
                .into_iter()
                .zip(values.into_iter())
                .map(|(name, value)| Value::NamedValue(String::from(name), String::from(value)))
                .collect::<Vec<_>>();
            Ok((left, values))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct InsertCommand {
    pub table_name: String,
    pub values: Vec<Value>,
}

impl NomParsable for InsertCommand {
    fn nom_parse(input: &str) -> nom::IResult<&str, Self> {
        let (left, (_, _, _, _, table_name, _, values)) = tuple((
            tag_no_case("insert"),
            space1,
            tag_no_case("into"),
            space1,
            alphanumeric1,
            space1,
            parse_value_list,
        ))(input.trim())?;

        Ok((
            left,
            InsertCommand {
                table_name: String::from(table_name),
                values,
            },
        ))
    }
}

impl InsertCommand {
    fn new(table_name: impl Into<String>, values: Vec<Value>) -> Self {
        InsertCommand {
            table_name: table_name.into(),
            values,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::parser::{command::insert_command::Value, NomParsable};

    use super::InsertCommand;

    #[test]
    fn parse_test_with_value() -> Result<(), String> {
        let command = "insert into test VALUEs (one, two, 1234, five);";
        let insert_command = InsertCommand::nom_parse(command)
            .map_err(|err| format!("Failed with error {:?}", err))?;

        assert_eq!(
            insert_command.1,
            InsertCommand::new(
                "test",
                vec![
                    Value::new_unnamed_value("one"),
                    Value::new_unnamed_value("two"),
                    Value::new_unnamed_value("1234"),
                    Value::new_unnamed_value("five")
                ]
            )
        );
        Ok(())
    }

    #[test]
    fn parse_test_with_name_value() -> Result<(), String> {
        let command = "insert into test (col1, col2, col4, col3) VALUEs (one, two, 1234, five);";
        let insert_command = InsertCommand::nom_parse(command)
            .map_err(|err| format!("Failed with error {:?}", err))?;

        assert_eq!(
            insert_command.1,
            InsertCommand::new(
                "test",
                vec![
                    Value::new_named_value("col1", "one"),
                    Value::new_named_value("col2", "two"),
                    Value::new_named_value("col4", "1234"),
                    Value::new_named_value("col3", "five")
                ]
            )
        );
        Ok(())
    }
}
