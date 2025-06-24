use crate::config;

pub fn execute_list() -> Result<(), String> {
    //Remove example
    //app-tree remove my-awesome-project
    let actions_result = config::read_config();
    return match actions_result {
        Ok(actions) => {
            println!("NAME --- ACTION");
            for action in actions.iter() {
                println!("{} --- {}", action.name, action.command)
            }
            Ok(())
        }
        Err(e) => Err(format!("{e}")),
    };
}
