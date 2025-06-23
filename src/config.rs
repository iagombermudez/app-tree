use crate::models::app_leaf::AppLeaf;
use serde_json::Result;
use std::io::Write;
use std::{fs::File, io::BufWriter};

pub fn read_config() -> Result<Vec<AppLeaf>> {
    let apps_file = match File::open("app_config.json") {
        Ok(file) => file,
        Err(_) => File::create("app_config.json").expect("File could not be created"),
    };
    if apps_file.metadata().unwrap().len() == 0 {
        return Ok(Vec::new());
    };
    let apps: Vec<AppLeaf> = serde_json::from_reader(apps_file).expect("JSON could not be parsed");
    return Ok(apps);
}

pub fn write_config(config: Vec<AppLeaf>) {
    let mut apps_file = match File::options()
        .read(true)
        .write(true)
        .open("app_config.json")
    {
        Ok(file) => file,
        Err(_) => File::create("app_config.json").expect("File could not be created"),
    };
    let result_writter_json = serde_json::to_writer(&mut apps_file, &config);
    match result_writter_json {
        Ok(_) => {
            let mut writer = BufWriter::new(apps_file);
            let writer_result = writer.flush();
            match writer_result {
                Ok(_) => {}
                Err(_) => {}
            };
        }
        Err(error) => {
            println!("{error}");
            panic!("File could not be parsed to json")
        }
    };
}
