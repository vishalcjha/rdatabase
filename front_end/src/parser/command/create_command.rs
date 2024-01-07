use crate::parser::{table_defination::TableDefination, NomParsable};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct CreateCommand(TableDefination);

impl NomParsable for CreateCommand {
    fn nom_parse(input: &str) -> nom::IResult<&str, Self> {
        let (left, table_defination) = TableDefination::nom_parse(input)?;

        Ok((left, CreateCommand(table_defination)))
    }
}
