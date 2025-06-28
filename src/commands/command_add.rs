use crate::{
    config,
    models::action::{ActionComponent, ActionComposite, ActionLeaf},
};

pub fn execute_add() -> Result<(), String> {
    let actions_result = config::read_config();
    match actions_result {
        Ok(mut actions) => {
            // Need to handle branch names. Right now, we will only ned to handle
            // one, but in the future, there might be N branches.
            // Example: app-tree add repos my-repo "executable"

            // Read the app_name and executable arguments
            let num_args = std::env::args().len();
            let action_branch: String;
            let action_name = std::env::args()
                .nth(std::env::args().len() - 2)
                .expect("Action name parameter is missing");
            let action_command = std::env::args()
                .nth(std::env::args().len() - 1)
                .expect("Executable parameter is missing");

            match num_args {
                4 => {
                    // Check if the app already exists
                    let action_exists: bool = actions.iter().any(|action| match action {
                        ActionComponent::Leaf(leaf) => leaf.name == action_name,
                        ActionComponent::Component(component) => component.name == action_name,
                    });

                    if action_exists {
                        return Err(format!("Command {} already exists", action_name));
                    }
                    //Add a new AppLeaf
                    let action =
                        ActionComponent::Leaf(ActionLeaf::new(action_name, action_command));
                    actions.push(action);
                }
                5 => {
                    action_branch = std::env::args()
                        .nth(2)
                        .expect("Action branch name parameter is missing");
                    // Check if the action already exists
                    let action_exists: bool = actions.iter().any(|action| match action {
                        ActionComponent::Leaf(leaf) => leaf.name == &*action_name,
                        ActionComponent::Component(component) => {
                            component.name == action_branch
                                && component.actions.iter().any(|action| match action {
                                    ActionComponent::Leaf(leaf) => leaf.name == &*action_name,
                                    ActionComponent::Component(component) => {
                                        component.name == action_name
                                    }
                                })
                        }
                    });
                    if action_exists {
                        return Err(format!("Command {} already exists", action_name));
                    }

                    //Create new AppLeaf
                    let new_leaf = ActionComponent::Leaf(ActionLeaf {
                        name: action_name.clone(),
                        command: action_command,
                    });
                    let branch = actions.iter().find(|action| match action {
                        ActionComponent::Leaf(leaf) => leaf.name == action_name,
                        ActionComponent::Component(component) => component.name == action_name,
                    });

                    let action_component: ActionComponent = match branch {
                        Some(branch) => match branch {
                            ActionComponent::Leaf(_) => branch.clone(),
                            ActionComponent::Component(component) => {
                                let mut composite = component.clone();
                                composite.add(new_leaf);
                                let new_branch = ActionComponent::Component(composite);
                                new_branch
                            }
                        },
                        _ => {
                            let mut composite = ActionComposite {
                                name: action_branch,
                                actions: actions.clone(),
                            };
                            composite.add(new_leaf);
                            let new_branch = ActionComponent::Component(composite);
                            new_branch
                        }
                    };
                    actions.push(action_component)
                }
                _ => panic!("Incorrect number of args"),
            };
            let save_result = config::write_config(actions.clone());
            return save_result;
        }
        Err(e) => Err(format!("Error {}", e)),
    }
}
