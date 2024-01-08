pub mod parser;
pub type FEResult<T> = anyhow::Result<T>;
pub use parser::command::create_command::CreateCommand;
pub use parser::command_type::CommandType;
pub use parser::table_defination;
