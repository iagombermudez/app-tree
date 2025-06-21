use std::process::Command;

use crate::models::app_leaf::AppLeaf;

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

pub fn execute_open(apps: Vec<AppLeaf>) {
    let app_arg = std::env::args().nth(2).expect("App parameter is missing");
    //Look for the app and execute the command if found
    //TODO: find a way to see how to match vectors
    for app_leaf in &apps {
        if app_leaf.app_name == app_arg {
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
    panic!("App {} was not found", app_arg);
}
