use crate::{
    config,
    models::action::{Action, ActionComponent},
};

pub fn execute_list() -> Result<(), String> {
    let actions_result = config::read_config();
    return match actions_result {
        Ok(actions) => {
            println!("NAME --- ACTION");
            for action in actions.iter() {
                match action {
                    ActionComponent::Leaf(leaf) => println!("{}", leaf.to_string(0)),
                    ActionComponent::Branch(branch) => println!("{}", branch.to_string(0)),
                }
            }
            Ok(())
        }
        Err(e) => Err(format!("{e}")),
    };
}
