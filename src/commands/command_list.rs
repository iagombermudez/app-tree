use crate::config;

pub fn execute_list() -> Result<(), String> {
    let actions_result = config::read_config();
    return match actions_result {
        Ok(actions) => {
            println!("NAME --- ACTION");
            for action in actions.iter() {
                action.print()
            }
            Ok(())
        }
        Err(e) => Err(format!("{e}")),
    };
}
