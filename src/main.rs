mod commands;
mod config;
mod models;

use crate::commands::AppCommand;

fn main() {
    // Read the command arguments
    // A command will follow the following pattern
    //     app-tree <branch> <app>
    // For example, app-tree repos my-awesome-project
    // Get the command first. Possible arguments are add/open.
    // Also, handle incorrect command too
    let app_command: AppCommand = commands::read_app_command();
    match app_command {
        AppCommand::Add => {
            let add_result = commands::execute_add();
            match add_result {
                Ok(_) => println!("Element added correctly"),
                Err(error) => println!("{error}"),
            }
        }
        AppCommand::Open => commands::execute_open(),
        AppCommand::Incorrect => {
            println!("Incorrect command. Allowed commands are open | add")
        }
    };
}
