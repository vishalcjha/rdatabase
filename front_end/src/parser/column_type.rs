use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag_no_case,
    character::complete::{alphanumeric1, space0, space1},
    sequence::tuple,
    IResult,
};

use super::{NomParsable, Sizable};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ColumnType {
    Int,
    TEXT,
}

impl Sizable for ColumnType {
    fn byte_size(&self) -> u32 {
        match self {
            ColumnType::Int => 8,
            // Size + pointer to data
            ColumnType::TEXT => 12,
        }
    }
}

impl FromStr for ColumnType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ColumnType::*;
        let input = s.to_lowercase();
        match input.as_str() {
            "int" => Ok(Int),
            "text" => Ok(TEXT),
            _ => Err(anyhow::anyhow!(format!(
                "Failed to parse {} to ColumnType",
                s
            ))),
        }
    }
}

impl NomParsable for ColumnType {
    fn nom_parse(input: &str) -> IResult<&str, ColumnType> {
        let (left, type_name) = alt((tag_no_case("int"), tag_no_case("text")))(input)?;
        Ok((left, ColumnType::from_str(type_name).unwrap()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Column(String, ColumnType);

impl Column {
    pub fn new(name: impl Into<String>, column_type: ColumnType) -> Column {
        Column(name.into(), column_type)
    }
}

impl NomParsable for Column {
    fn nom_parse(input: &str) -> IResult<&str, Self> {
        let (left, (_, name, _, col_type)) =
            tuple((space0, alphanumeric1, space1, ColumnType::nom_parse))(input)?;

        Ok((left, Column(String::from(name), col_type)))
    }
}
