pub mod parser;
pub type FEResult<T> = anyhow::Result<T>;
pub use parser::command::CommandType;
