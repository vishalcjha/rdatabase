use nom::IResult;

pub mod column_type;
pub mod command;
pub mod table_defination;

pub use command::command_type;
trait Sizable {
    fn byte_size(&self) -> u32;
}

trait NomParsable {
    fn nom_parse(input: &str) -> IResult<&str, Self>
    where
        Self: Sized;
}
