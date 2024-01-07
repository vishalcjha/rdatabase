use std::str::FromStr;

use anyhow::{anyhow, bail};
use nom::{branch::alt, bytes::complete::tag_no_case, IResult};

use crate::parser::NomParsable;

use super::{
    create_command::CreateCommand, insert_command::InsertCommand, select_command::SelectCommand,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CommandType {
    Select(SelectCommand),
    Insert(InsertCommand),
    Delete(String),
    CREATE(CreateCommand),
}

fn parse_command_line(input: &str) -> IResult<&str, String> {
    let (left, operation) = alt((
        tag_no_case("select"),
        tag_no_case("insert"),
        tag_no_case("delete"),
        tag_no_case("create"),
    ))(input)?;

    Ok((left, operation.to_lowercase()))
}

impl FromStr for CommandType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CommandType::*;
        let command_type = parse_command_line(s)
            .map_err(|err| anyhow!(format!("Parse command failed with {:?}", err)))?
            .1;
        let lowercase_input = s.to_lowercase();

        match command_type.as_str() {
            "select" => {
                let select_command = SelectCommand::nom_parse(&lowercase_input)
                    .map_err(|err| anyhow!("{:?}", err))?;
                Ok(Select(select_command.1))
            }
            "delete" => Ok(Delete(s.to_lowercase())),
            "insert" => {
                let insert_command = InsertCommand::nom_parse(&lowercase_input)
                    .map_err(|err| anyhow!("{:?}", err))?;
                Ok(Insert(insert_command.1))
            }
            "create" => {
                let create_command = CreateCommand::nom_parse(&lowercase_input)
                    .map_err(|err| anyhow!("{:?}", err))?;
                Ok(CREATE(create_command.1))
            }
            _ => bail!("Failed to parse command {:?}", s),
        }
    }
}

#[cfg(test)]
mod test {
    use rstest::*;
    use std::str::FromStr;

    use crate::parser::{
        command::{
            create_command::CreateCommand, insert_command::InsertCommand,
            select_command::SelectCommand,
        },
        NomParsable,
    };

    use super::CommandType;

    #[rstest]
    #[case("select * from test;", CommandType::Select(SelectCommand::nom_parse("select * from test;").unwrap().1))]
    #[case(
        "delete from test where 1 = 1;",
        CommandType::Delete(String::from("delete from test where 1 = 1;"))
    )]
    #[case(
        "insert into test values(1, 2, 3);",
        CommandType::Insert(InsertCommand::nom_parse("insert into test values(1, 2, 3);")
        .unwrap()
        .1)
    )]
    #[case(
        "create table test (c1 int, c2 TEXT);",
        CommandType::CREATE(CreateCommand::nom_parse("create table test (c1 int, c2 TEXT);")
        .unwrap()
        .1)
    )]
    fn test_success(#[case] command: &'static str, #[case] expected: CommandType) {
        let Ok(command) = CommandType::from_str(command) else {
            panic!("Test failed for {}", command);
        };
        assert_eq!(command, expected);
    }
}
