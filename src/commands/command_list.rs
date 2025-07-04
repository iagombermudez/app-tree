use crate::{
    config,
    models::action::{Action, ActionComponent},
};

pub fn execute_list() -> Result<(), String> {
    let actions_result = config::read_config();
    return match actions_result {
        Ok(actions) => {
            if actions.len() == 0 {
                println!("No actions. Add an action by using the <add> command");
            } else {
                println!("NAME --- ACTION");
                for action in actions.iter() {
                    match action {
                        ActionComponent::Leaf(leaf) => println!("{}", leaf.to_string(0)),
                        ActionComponent::Branch(branch) => println!("{}", branch.to_string(0)),
                    }
                }
            }

            Ok(())
        }
        Err(e) => Err(format!("{e}")),
    };
}
