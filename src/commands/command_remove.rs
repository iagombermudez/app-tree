use crate::{config, models::action::ActionComponent};

pub fn execute_remove() -> Result<(), String> {
    //Remove example
    //app-tree remove my-awesome-project
    let actions_result = config::read_config();
    return match actions_result {
        Ok(mut actions) => {
            let action_name = std::env::args()
                .nth(2)
                .expect("Action name parameter is missing");

            //Look for the app and execute the command if found
            let find_action_result = actions.iter().position(|action| match action {
                ActionComponent::Leaf(leaf) => leaf.name == action_name,
                ActionComponent::Component(component) => component.name == action_name,
            });
            return match find_action_result {
                Some(action_index) => {
                    actions.remove(action_index);
                    return config::write_config(actions);
                }
                None => Err(format!("Action {} was not found", action_name)),
            };
        }
        Err(e) => Err(format!("{e}")),
    };
}
