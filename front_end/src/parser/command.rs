use std::str::FromStr;

use anyhow::{anyhow, bail};
use nom::{branch::alt, bytes::complete::tag_no_case, IResult};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CommandType {
    Select(String),
    Insert(String),
    Delete(String),
    CREATE(String),
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
        match command_type.as_str() {
            "select" => Ok(Select(s.to_lowercase())),
            "delete" => Ok(Delete(s.to_lowercase())),
            "insert" => Ok(Insert(s.to_lowercase())),
            "create" => Ok(CREATE(s.to_lowercase())),
            _ => bail!("Failed to parse command {:?}", s),
        }
    }
}

#[cfg(test)]
mod test {
    use rstest::*;
    use std::str::FromStr;

    use super::CommandType;

    #[rstest]
    #[case(
        "select * from test;",
        CommandType::Select(String::from("select * from test;"))
    )]
    #[case(
        "delete from test where 1 = 1;",
        CommandType::Delete(String::from("delete from test where 1 = 1;"))
    )]
    #[case(
        "insert into test values(1, 2, 3);",
        CommandType::Insert(String::from("insert into test values(1, 2, 3);"))
    )]
    #[case(
        "create table test (c1 int, c2 varchar(200));",
        CommandType::CREATE(String::from("create table test (c1 int, c2 varchar(200));"))
    )]
    fn test_success(#[case] command: &'static str, #[case] expected: CommandType) {
        let Ok(command) = CommandType::from_str(command) else {
            panic!("Test failed for {}", command);
        };
        assert_eq!(command, expected);
    }
}
