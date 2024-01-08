use std::{str::FromStr, sync::Mutex};

use front_end::{table_defination::TableDefination, CommandType, CreateCommand};
use lazy_static::lazy_static;

use super::StorageResult;

lazy_static! {
    pub static ref STORAGE_MANAGER: StorageManager = StorageManager {
        table_definations: Mutex::default(),
    };
}

#[derive(Debug)]
pub struct StorageManager {
    table_definations: Mutex<Vec<TableDefination>>,
}

impl StorageManager {
    pub fn execute_command(&self, command: impl AsRef<str>) -> StorageResult<()> {
        let command = CommandType::from_str(command.as_ref())?;
        match command {
            CommandType::Select(_) => todo!(),
            CommandType::Insert(_) => todo!(),
            CommandType::Delete(_) => todo!(),
            CommandType::CREATE(create_command) => self.execute_create_table(create_command)?,
        };
        Ok(())
    }

    fn execute_create_table(&self, _create_command: CreateCommand) -> StorageResult<()> {
        Ok(())
    }
}
