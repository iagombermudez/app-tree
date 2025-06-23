use std::process::Command;

use crate::{config, models::app_leaf::AppLeaf};

pub enum AppCommand {
    Add,
    Open,
    Incorrect,
}

pub fn read_app_command() -> AppCommand {
    let command_arg = std::env::args()
        .nth(1)
        .expect("Command parameter is missing");
    let command = match command_arg {
        _ if command_arg == "add" => AppCommand::Add,
        _ if command_arg == "open" => AppCommand::Open,
        _ => AppCommand::Incorrect,
    };
    return command;
}

pub fn execute_add() -> Result<(), String> {
    let apps_result = config::read_config();
    match apps_result {
        Ok(mut apps) => {
            // Read the app_name and executable arguments
            let app_name_arg = std::env::args()
                .nth(2)
                .expect("App name parameter is missing");
            let executable_arg = std::env::args()
                .nth(3)
                .expect("Executable parameter is missing");

            // Check if the app already exists
            let app_exists: bool = apps.iter().any(|app| app.app_name == app_name_arg);
            if app_exists {
                return Err(format!("Command {} already exists", app_name_arg));
            }
            //Create new AppLeaf
            let app_leaf = AppLeaf {
                app_name: app_name_arg,
                executable: executable_arg,
            };

            // Add the new instance of app_leaf to apps
            apps.push(app_leaf);

            // Save the new app in the configuration file
            config::write_config(apps);
            return Ok(());
        }
        Err(e) => Err(format!("Error {}", e)),
    }
}
pub fn execute_open() {
    let apps_result = config::read_config();
    match apps_result {
        Ok(apps) => {
            let app_name_arg = std::env::args()
                .nth(2)
                .expect("App name parameter is missing");
            //Look for the app and execute the command if found
            //TODO: find a way to see how to match vectors
            for app_leaf in &apps {
                if app_leaf.app_name == app_name_arg {
                    let command_execution_result = Command::new(&app_leaf.executable).spawn();
                    match command_execution_result {
                        Err(error) => panic!("There was an error executing app `{}`: {error}", {
                            &app_leaf.executable
                        }),
                        _ => {}
                    };
                    break;
                }
            }
            panic!("App {} was not found", app_name_arg);
        }
        Err(e) => println!("Error {}", e),
    }
}
