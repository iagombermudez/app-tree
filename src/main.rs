mod commands;
mod config;
mod models;

use crate::commands::AppCommand;

fn main() {
    // Read the command arguments
    // A command will follow the following pattern
    //     app-tree <action> <args..N>
    // For example, app-tree open repos my-awesome-project

    // Get the command first. Possible arguments are add/open.
    // Also, handle incorrect command too
    let app_command: AppCommand = commands::read_app_command();
    match app_command {
        AppCommand::Add => {
            let execute_add_result = commands::execute_add();
            if let Err(error) = execute_add_result {
                println!("{error}");
            }
        }
        AppCommand::Open => {
            let execute_open_result = commands::execute_open();
            if let Err(error) = execute_open_result {
                println!("{error}")
            }
        }
        AppCommand::Remove => {
            let execute_remove_result = commands::execute_remove();
            if let Err(error) = execute_remove_result {
                println!("{error}")
            }
        }

        AppCommand::Incorrect => {
            println!("Incorrect command. Allowed commands are open | add")
        }
    };
}
