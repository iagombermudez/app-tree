use crate::models::action::{ActionComponent, ActionJSON, ActionLeaf};
use std::io::Write;
use std::{fs::File, io::BufWriter};

pub fn read_config() -> Result<Vec<ActionComponent>, String> {
    let apps_file = match File::open("app_config.json") {
        Ok(file) => file,
        Err(_) => File::create("app_config.json").expect("File could not be created"),
    };
    if apps_file.metadata().unwrap().len() == 0 {
        return Ok(Vec::new());
    };
    let apps_json: Vec<ActionJSON> =
        serde_json::from_reader(apps_file).expect("JSON could not be parsed");
    let mut actions: Vec<ActionComponent> = vec![];
    for app in apps_json.iter() {
        parse_to_actions(&mut actions, app);
    }
    return Ok(actions);
}

fn parse_to_actions(actions: &mut Vec<ActionComponent>, action: &ActionJSON) {
    match &action.command {
        Some(command) => actions.push(ActionComponent::Leaf(ActionLeaf::new(
            action.name.to_string(),
            command.to_string(),
        ))),
        _ => match &action.actions {
            Some(app_actions) => {
                for action in app_actions.iter() {
                    parse_to_actions(actions, action);
                }
            }
            _ => panic!("No command or branch provided, this should not happen"),
        },
    }
}

pub fn write_config(config: Vec<ActionComponent>) -> Result<(), String> {
    let mut apps_file = match File::options()
        .read(true)
        .write(true)
        .truncate(true)
        .open("app_config.json")
    {
        Ok(file) => file,
        Err(_) => File::create("app_config.json").expect("File could not be created"),
    };

    let result_writter_json = serde_json::to_writer(&mut apps_file, &config);
    return match result_writter_json {
        Ok(_) => {
            let mut writer = BufWriter::new(apps_file);
            let writer_result = writer.flush();
            return match writer_result {
                Err(_) => Err(String::from("Error saving the configuration")),
                _ => Ok(()),
            };
        }
        Err(_) => Err(String::from("File could not be parsed to json")),
    };
}
