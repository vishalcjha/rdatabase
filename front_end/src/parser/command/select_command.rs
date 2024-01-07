use nom::{
    bytes::complete::{tag, tag_no_case, take_until1},
    character::complete::{alphanumeric1, space0, space1},
    combinator::opt,
    multi::separated_list1,
    sequence::tuple,
};

use crate::parser::NomParsable;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
enum Columns {
    All,
    Selected(Vec<String>),
}

impl NomParsable for Columns {
    fn nom_parse(input: &str) -> nom::IResult<&str, Self> {
        let (left, processed) = opt(tag("*"))(input)?;
        match (left, processed) {
            (left, Some(_)) => Ok((left, Columns::All)),
            (left, None) => {
                let (rest, columns) = take_until1(" from")(left)?;
                let (_, columns) =
                    separated_list1(tag(","), tuple((space0, alphanumeric1)))(columns)?;

                // neglect space in column tuple while iterating
                let columns = columns.into_iter().map(|it| String::from(it.1)).collect();

                Ok((rest, Columns::Selected(columns)))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct SelectCommand {
    table_name: String,
    columns: Columns,
}

impl NomParsable for SelectCommand {
    fn nom_parse(input: &str) -> nom::IResult<&str, Self> {
        let (left, (_, _, columns, _, _, _, table_name)) = tuple((
            tag_no_case("select"),
            space1,
            Columns::nom_parse,
            space1,
            tag_no_case("from"),
            space1,
            alphanumeric1,
        ))(input.trim())?;
        Ok((
            left,
            SelectCommand {
                table_name: String::from(table_name),
                columns,
            },
        ))
    }
}

#[cfg(test)]
mod test {
    use crate::parser::{command::select_command::Columns, NomParsable};

    use super::SelectCommand;

    #[test]
    fn test_selected_columns() -> Result<(), String> {
        let command = "Select col1, col3, col4 from  test;";
        let result = SelectCommand::nom_parse(command)
            .map_err(|err| format!("Processing select command failed with {:#?}", err))?;
        assert_eq!(
            result.1,
            SelectCommand {
                table_name: String::from("test"),
                columns: Columns::Selected(
                    vec!["col1", "col3", "col4"]
                        .into_iter()
                        .map(|it| it.to_string())
                        .collect()
                )
            }
        );

        Ok(())
    }

    #[test]
    fn test_all_columns() -> Result<(), String> {
        let command = "Select * from  test;";
        let result = SelectCommand::nom_parse(command)
            .map_err(|err| format!("Processing select command failed with {:#?}", err))?;
        assert_eq!(
            result.1,
            SelectCommand {
                table_name: String::from("test"),
                columns: Columns::All
            }
        );

        Ok(())
    }
}
