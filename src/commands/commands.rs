pub enum AppCommand {
    Help,
    Add,
    Execute,
    Remove,
    List,
    Incorrect,
}

pub fn read_app_command() -> AppCommand {
    let command_arg = std::env::args()
        .nth(1)
        .expect("Command parameter is missing");
    let command = match command_arg {
        _ if command_arg == "--help" || command_arg == "-h" => AppCommand::Help,
        _ if command_arg == "add" => AppCommand::Add,
        _ if command_arg == "execute" => AppCommand::Execute,
        _ if command_arg == "remove" => AppCommand::Remove,
        _ if command_arg == "list" => AppCommand::List,
        _ => AppCommand::Incorrect,
    };
    return command;
}
