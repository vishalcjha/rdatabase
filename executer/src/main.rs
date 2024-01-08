use rustyline::{error::ReadlineError, DefaultEditor};

fn main() {
    println!("Starting database");
    let storage_manager = &storage::in_memory::manager::STORAGE_MANAGER;
    let mut rl = DefaultEditor::new().unwrap();

    loop {
        let readline = rl.readline("db > ");
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                match storage_manager.execute_command(&line) {
                    Ok(_) => println!("Execution successful"),
                    Err(_) => println!("Failed to execute "),
                };
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(_) => panic!("Failed to read line. Aboring !!!"),
        }
    }

    //#[cfg(feature = "with-file-history")]
    let _ = rl.save_history("history.txt");
}
