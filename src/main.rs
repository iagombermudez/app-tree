mod commands;
mod config;
mod models;
use crate::commands::AppCommand;

fn main() {
    let apps_result = config::read_config();
    match apps_result {
        Ok(apps) => {
            // Read the command arguments
            // A command will follow the following pattern
            //     app-tree <branch> <app>
            // For example, app-tree repos my-awesome-project
            //Get the command first. Possible arguments are add/open/remove
            let app_command: AppCommand = commands::read_app_command();
            match app_command {
                AppCommand::Add => println!("Command add"),
                AppCommand::Open => commands::execute_open(apps),
                AppCommand::Incorrect => {
                    println!("Incorrect command. Allowed commands are open | add")
                }
            };
        }
        Err(e) => println!("Error {}", e),
    }
}
