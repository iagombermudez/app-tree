pub enum AppCommand {
    Add,
    Open,
    Remove,
    List,
    Incorrect,
}

pub fn read_app_command() -> AppCommand {
    let command_arg = std::env::args()
        .nth(1)
        .expect("Command parameter is missing");
    let command = match command_arg {
        _ if command_arg == "add" => AppCommand::Add,
        _ if command_arg == "open" => AppCommand::Open,
        _ if command_arg == "remove" => AppCommand::Remove,
        _ if command_arg == "list" => AppCommand::List,
        _ => AppCommand::Incorrect,
    };
    return command;
}
