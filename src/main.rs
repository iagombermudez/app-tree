use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Serialize, Deserialize)]
struct AppLeaf {
    branch: String,
    app_name: String,
    executable: String,
}

fn main() {
    //Read file with apps configuration
    let apps_file = match File::open("app_config.json") {
        Ok(file) => file,
        Err(_) => File::create("app_config.json").expect("File could not be created"),
    };
    let apps: Vec<AppLeaf> = serde_json::from_reader(apps_file).expect("JSON could not be parsed");

    // Read the command arguments
    // A command will follow the following pattern
    //     app-tree <branch> <app>
    // For example, app-tree repos my-awesome-project
    let branch = std::env::args()
        .nth(1)
        .expect("Branch parameter is missing");
    let app = std::env::args().nth(2).expect("App parameter is missing");
    println!("Parameters given: {branch} / {app}");

    //Look for the app and execute the command if found
    let mut app_found: bool = false;
    for x in &apps {
        if x.branch == branch && x.app_name == app {
            app_found = true;
            println!("{}", x.executable);
        }
    }

    if !app_found {
        println!("App {} was not found", app);
    }
}
