use crate::{
    config,
    models::action::{Action, ActionComponent},
};

pub fn execute_open() -> Result<(), std::string::String> {
    let actions_result = config::read_config();
    return match actions_result {
        Ok(actions) => {
            let action_name = std::env::args()
                .nth(2)
                .expect("Action name parameter is missing");

            //Look for the app and execute the command if found
            let find_action_result = actions.iter().find(|action| match action {
                ActionComponent::Leaf(leaf) => leaf.name == action_name,
                ActionComponent::Branch(branch) => branch.name == action_name,
            });
            return match find_action_result {
                Some(action) => {
                    match action {
                        ActionComponent::Leaf(leaf) => leaf.execute(),
                        ActionComponent::Branch(branch) => branch.execute(),
                    }
                    return Ok(());
                }
                None => Err(format!("Action {} was not found", action_name)),
            };
        }
        Err(e) => Err(format!("{e}")),
    };
}
