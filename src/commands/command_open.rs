use std::process::Command;

use crate::config;

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
