use crate::{
    config,
    models::action::{ActionBranch, ActionComponent, ActionLeaf},
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
            let action_name = std::env::args()
                .nth(std::env::args().len() - 2)
                .expect("Action name parameter is missing");
            let action_command = std::env::args()
                .nth(std::env::args().len() - 1)
                .expect("Executable parameter is missing");

            match num_args {
                4 => add_action(&mut actions, &action_name, &action_command),
                5 => add_action_to_branch(&mut actions, action_name, action_command),
                _ => panic!("Incorrect number of args"),
            };
            let save_result = config::write_config(actions.clone());
            return save_result;
        }
        Err(e) => Err(format!("Error {}", e)),
    }
}

fn add_action(
    actions: &mut Vec<ActionComponent>,
    action_name: &String,
    action_command: &String,
) -> Option<Result<(), String>> {
    let action_exists: bool = actions.iter().any(|action| match action {
        ActionComponent::Leaf(leaf) => leaf.name == *action_name,
        ActionComponent::Branch(_) => false,
    });
    if action_exists {
        return Some(Err(format!("Command {} already exists", action_name)));
    }
    let action =
        ActionComponent::Leaf(ActionLeaf::new(action_name.clone(), action_command.clone()));
    actions.push(action);
    None
}

fn add_action_to_branch(
    actions: &mut Vec<ActionComponent>,
    action_name: String,
    action_command: String,
) -> Option<Result<(), String>> {
    let branch_name = std::env::args()
        .nth(2)
        .expect("Action branch name parameter is missing");
    let action_exists: bool = actions.iter().any(|action| match action {
        ActionComponent::Leaf(leaf) => leaf.name == &*action_name,
        ActionComponent::Branch(branch) => {
            branch.name == branch_name
                && branch.actions.iter().any(|action| match action {
                    ActionComponent::Leaf(leaf) => leaf.name == &*action_name,
                    ActionComponent::Branch(_) => false,
                })
        }
    });
    if action_exists {
        return Some(Err(format!("Action {} already exists", action_name)));
    }
    let new_leaf = ActionComponent::Leaf(ActionLeaf {
        name: action_name.clone(),
        command: action_command,
    });
    let find_branch_result = actions.iter().position(|action| match action {
        ActionComponent::Branch(branch) => branch.name == branch_name,
        _ => false,
    });
    match find_branch_result {
        Some(branch_position) => {
            let branch_component = &actions[branch_position];
            match branch_component {
                ActionComponent::Branch(branch) => {
                    let mut branch_clone = branch.clone();
                    branch_clone.add(new_leaf);
                    actions[branch_position] = ActionComponent::Branch(branch_clone);
                }
                _ => panic!("This should be a branch"),
            }
        }
        _ => {
            let mut composite = ActionBranch {
                name: branch_name,
                actions: actions.clone(),
            };
            composite.add(new_leaf);
            let new_branch = ActionComponent::Branch(composite);
            actions.push(new_branch)
        }
    };
    None
}
