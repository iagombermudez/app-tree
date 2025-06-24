use crate::{config, models::action::Action};

pub fn execute_add() -> Result<(), String> {
    let actions_result = config::read_config();
    match actions_result {
        Ok(mut actions) => {
            // Read the app_name and executable arguments
            let action_name = std::env::args()
                .nth(2)
                .expect("Action name parameter is missing");
            let action_command = std::env::args()
                .nth(3)
                .expect("Executable parameter is missing");

            // Check if the app already exists
            let action_exists: bool = actions.iter().any(|app| app.name == action_name);
            if action_exists {
                return Err(format!("Command {} already exists", action_name));
            }
            //Create new AppLeaf
            let action = Action {
                name: action_name,
                command: action_command,
            };

            // Add the new instance of app_leaf to apps
            actions.push(action);

            // Save the new app in the configuration file
            let save_result = config::write_config(actions);
            return save_result;
        }
        Err(e) => Err(format!("Error {}", e)),
    }
}
