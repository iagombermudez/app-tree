mod commands;
mod config;
mod models;
use crate::commands::commands::AppCommand;

fn main() {
    // Read the command arguments
    // A command will follow the following pattern
    //     app-tree <action> <args..N>
    // For example, app-tree execute repos my-awesome-project

    // Get the command first. Possible arguments are add/execute.
    // Also, handle incorrect command too
    let app_command: AppCommand = commands::commands::read_app_command();
    match app_command {
        AppCommand::Add => {
            let execute_add_result = commands::command_add::execute();
            if let Err(error) = execute_add_result {
                println!("{error}");
            }
        }
        AppCommand::Execute => {
            let execute_execute_result = commands::command_execute::execute();
            if let Err(error) = execute_execute_result {
                println!("{error}")
            }
        }
        AppCommand::Remove => {
            let execute_remove_result = commands::command_remove::execute();
            if let Err(error) = execute_remove_result {
                println!("{error}")
            }
        }
        AppCommand::List => {
            let execute_remove_result = commands::command_list::execute();
            if let Err(error) = execute_remove_result {
                println!("{error}")
            }
        }
        AppCommand::Help => {
            let execute_help_result = commands::command_help::execute();
            if let Err(error) = execute_help_result {
                println!("{error}")
            }
        }
        AppCommand::Incorrect => {
            println!(
                "Incorrect command. Run 'app-tree --help' for more information on how to use the CLI."
            )
        }
    };
}
