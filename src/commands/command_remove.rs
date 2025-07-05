use crate::{config, models::action::ActionComponent};

pub fn execute() -> Result<(), String> {
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
                    let find_branch_position = actions.iter().position(|action| match action {
                        ActionComponent::Branch(branch) => branch.name == branch_name,
                        _ => false,
                    });

                    if let Some(branch_position) = find_branch_position {
                        let mut find_branch_result = actions.iter().find(|action| match action {
                            ActionComponent::Branch(branch) => branch.name == branch_name,
                            _ => false,
                        });

                        if let Some(branch_component) = &mut find_branch_result {
                            match branch_component {
                                ActionComponent::Branch(branch) => {
                                    let find_action_result =
                                        branch.actions.iter().position(|action| match action {
                                            ActionComponent::Leaf(leaf) => leaf.name == action_name,
                                            _ => false,
                                        });
                                    if let Some(action_position) = find_action_result {
                                        let mut cloned_branch = branch.clone();
                                        cloned_branch.actions.remove(action_position);
                                        actions.remove(branch_position);
                                        if cloned_branch.actions.len() > 0 {
                                            actions.push(ActionComponent::Branch(cloned_branch));
                                        }
                                    } else {
                                        panic!("PANICO");
                                    }
                                }
                                _ => panic!("PANICO"),
                            };
                        }

                        return config::write_config(actions);
                    } else {
                        panic!("PANICO")
                    }
                }
                _ => panic!("Incorrect number of args"),
            }
        }
        Err(e) => Err(format!("{e}")),
    };
}
