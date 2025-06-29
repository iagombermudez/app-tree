use crate::{config, models::action::ActionComponent};

pub fn execute_remove() -> Result<(), String> {
    //Remove example
    //app-tree remove my-awesome-project
    let actions_result = config::read_config();
    return match actions_result {
        Ok(mut actions) => {
            let num_args = std::env::args().len();
            match num_args {
                3 => {
                    let action_name = std::env::args()
                        .nth(2)
                        .expect("Action name parameter is missing");

                    //Look for the app and execute the command if found
                    let find_action_result = actions.iter().position(|action| match action {
                        ActionComponent::Leaf(leaf) => leaf.name == action_name,
                        ActionComponent::Branch(branch) => branch.name == action_name,
                    });
                    return match find_action_result {
                        Some(action_index) => {
                            actions.remove(action_index);
                            return config::write_config(actions);
                        }
                        None => Err(format!("Action {} was not found", action_name)),
                    };
                }
                4 => {
                    let branch_name = std::env::args()
                        .nth(2)
                        .expect("Branch name parameter is missing");
                    let action_name = std::env::args()
                        .nth(3)
                        .expect("Action name parameter is missing");

                    //Look for the app and execute the command if found
                    let find_branch_result = actions.iter().find(|action| match action {
                        ActionComponent::Branch(branch) => branch.name == branch_name,
                        _ => false,
                    });
                    return match find_branch_result {
                        Some(branch_component) => {
                            let find_action_result = match branch_component {
                                ActionComponent::Branch(branch) => {
                                    branch.actions.iter().position(|action| match action {
                                        ActionComponent::Leaf(leaf) => leaf.name == action_name,
                                        _ => false,
                                    })
                                }
                                _ => None,
                            };
                            match find_action_result {
                                Some(action_position) => actions.remove(action_position),
                                None => panic!("Action not found"),
                            };

                            return config::write_config(actions);
                        }
                        None => Err(format!("Action {} was not found", action_name)),
                    };
                }
                _ => panic!("Incorrect number of args"),
            }
        }
        Err(e) => Err(format!("{e}")),
    };
}
