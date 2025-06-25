use crate::{
    config,
    models::action::{ActionComposite, ActionStruct},
};

pub fn execute_add() -> Result<(), String> {
    let actions_result = config::read_config();
    match actions_result {
        Ok(mut actions) => {
            // Need to handle branch names. Right now, we will only need to handle
            // one, but in the future, there might be N branches.
            // Example: app-tree add repos my-repo "executable"
            // Read the app_name and executable arguments
            let num_args = std::env::args().len();
            let action_branch: String;
            let action_name: String;
            let action_command: String;
            match num_args {
                4 => {
                    action_name = std::env::args()
                        .nth(2)
                        .expect("Action name parameter is missing");
                    action_command = std::env::args()
                        .nth(3)
                        .expect("Executable parameter is missing");
                    // Check if the app already exists
                    let action_exists: bool =
                        actions.iter().any(|app| app.get_name() == action_name);
                    if action_exists {
                        return Err(format!("Command {} already exists", action_name));
                    }
                    //Create new AppLeaf
                    let action = ActionStruct::new(action_name, action_command);

                    // Add the new instance of app_leaf to apps
                    actions.push(Box::new(action));

                    // Save the new app in the configuration file
                    let save_result = config::write_config(actions);
                    return save_result;
                }
                5 => {
                    action_branch = std::env::args()
                        .nth(2)
                        .expect("Action branch name parameter is missing");

                    action_name = std::env::args()
                        .nth(3)
                        .expect("Action name parameter is missing");
                    action_command = std::env::args()
                        .nth(4)
                        .expect("Executable parameter is missing");
                    // Check if the app already exists
                    let action_exists: bool =
                        actions.iter().any(|app| app.get_name() == action_name);
                    if action_exists {
                        return Err(format!("Command {} already exists", action_name));
                    }

                    //Create new AppLeaf
                    let action = ActionStruct::new(action_name, action_command);
                    let branch = actions.iter().find(|a| a.get_name() == action_branch);
                    match branch {
                        Some(branch) => branch.add(action),
                        _ => {
                            let new_branch = ActionComposite::new(action_branch);
                            new_branch.add(action);
                        }
                    }
                    // Add the new instance of app_leaf to apps
                    actions.push(action);

                    // Save the new app in the configuration file
                    let save_result = config::write_config(actions);
                    return save_result;
                }
                _ => panic!("Incorrect number of args"),
            };
        }
        Err(e) => Err(format!("Error {}", e)),
    }
}
