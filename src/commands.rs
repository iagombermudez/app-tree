use crate::{config, models::action::Action};
use std::process::Command;

pub enum AppCommand {
    Add,
    Open,
    Remove,
    Incorrect,
}

pub fn read_app_command() -> AppCommand {
    let command_arg = std::env::args()
        .nth(1)
        .expect("Command parameter is missing");
    let command = match command_arg {
        _ if command_arg == "add" => AppCommand::Add,
        _ if command_arg == "open" => AppCommand::Open,
        _ if command_arg == "remove" => AppCommand::Remove,
        _ => AppCommand::Incorrect,
    };
    return command;
}

pub fn execute_add() -> Result<(), String> {
    let apps_result = config::read_config();
    match apps_result {
        Ok(mut apps) => {
            // Read the app_name and executable arguments
            let action_name = std::env::args()
                .nth(2)
                .expect("App name parameter is missing");
            let action_command = std::env::args()
                .nth(3)
                .expect("Executable parameter is missing");

            // Check if the app already exists
            let action_exists: bool = apps.iter().any(|app| app.name == action_name);
            if action_exists {
                return Err(format!("Command {} already exists", action_name));
            }
            //Create new AppLeaf
            let app_leaf = Action {
                name: action_name,
                command: action_command,
            };

            // Add the new instance of app_leaf to apps
            apps.push(app_leaf);

            // Save the new app in the configuration file
            let save_result = config::write_config(apps);
            return save_result;
        }
        Err(e) => Err(format!("Error {}", e)),
    }
}

pub fn execute_open() -> Result<(), std::string::String> {
    let actions_result = config::read_config();
    return match actions_result {
        Ok(actions) => {
            let action_name = std::env::args()
                .nth(2)
                .expect("Action name parameter is missing");

            //Look for the app and execute the command if found
            let find_action_result = actions.iter().find(|action| action.name == action_name);
            return match find_action_result {
                Some(action) => {
                    let command_execution_result = Command::new(&action.command).spawn();
                    if let Err(error) = command_execution_result {
                        return Err(format!("There was an error executing app `{}`: {error}", {
                            &action.command
                        }));
                    };
                    return Ok(());
                }
                None => Err(format!("Action {} was not found", action_name)),
            };
        }
        Err(e) => Err(format!("{e}")),
    };
}

pub fn execute_remove() -> Result<(), String> {
    //Remove example
    //app-tree remove my-awesome-project
    let actions_result = config::read_config();
    return match actions_result {
        Ok(mut actions) => {
            let action_name = std::env::args()
                .nth(2)
                .expect("Action name parameter is missing");

            //Look for the app and execute the command if found
            let find_action_result = actions.iter().position(|action| action.name == action_name);
            return match find_action_result {
                Some(action_index) => {
                    actions.remove(action_index);
                    return config::write_config(actions);
                }
                None => Err(format!("Action {} was not found", action_name)),
            };
        }
        Err(e) => Err(format!("{e}")),
    };
}
