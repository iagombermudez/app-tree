use crate::models::app_leaf::AppLeaf;
use serde_json::Result;
use std::fs::File;

pub fn read_config() -> Result<Vec<crate::models::app_leaf::AppLeaf>> {
    let apps_file = match File::open("app_config.json") {
        Ok(file) => file,
        Err(_) => File::create("app_config.json").expect("File could not be created"),
    };
    let apps: Vec<AppLeaf> = serde_json::from_reader(apps_file).expect("JSON could not be parsed");
    return Ok(apps);
}
