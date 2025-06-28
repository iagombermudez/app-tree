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
                    ActionComponent::Leaf(leaf) => leaf.print_ln(),
                    ActionComponent::Branch(branch) => branch.print_ln(),
                }
            }
            Ok(())
        }
        Err(e) => Err(format!("{e}")),
    };
}
