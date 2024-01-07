use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, space1},
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult,
};

use super::{column_type::Column, NomParsable};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub(super) struct TableDefination {
    name: String,
    columns: Vec<Column>,
}

fn parse_columns(columns: &str) -> IResult<&str, Vec<Column>> {
    let (left, columns) = delimited(
        tag("("),
        separated_list1(tag(","), Column::nom_parse),
        tag(")"),
    )(columns)?;
    Ok((left, columns))
}

impl NomParsable for TableDefination {
    fn nom_parse(input: &str) -> IResult<&str, Self> {
        let (left, (_, _, _, _, table_name, _, columns)) = tuple((
            tag("create"),
            space1,
            tag("table"),
            space1,
            alpha1,
            space1,
            parse_columns,
        ))(input)?;

        Ok((
            left,
            TableDefination {
                name: String::from(table_name),
                columns,
            },
        ))
    }
}

#[cfg(test)]
mod test {
    use crate::parser::{
        column_type::{Column, ColumnType},
        NomParsable,
    };

    use super::TableDefination;
    #[test]
    fn test_successful() -> Result<(), String> {
        let create_command = "create table    test ( col1 int, col2 text, col3 int);";

        let result =
            TableDefination::nom_parse(create_command).map_err(|err| format!("{:?}", err))?;

        assert_eq!(
            TableDefination {
                name: String::from("test"),
                columns: vec![
                    Column::new("col1", ColumnType::Int),
                    Column::new("col2", ColumnType::TEXT),
                    Column::new("col3", ColumnType::Int)
                ]
            },
            result.1
        );

        Ok(())
    }
}
