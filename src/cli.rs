use clap::{Arg, Command};

pub struct CLIArgs {
    pub command: String,
    pub service: Option<String>,
}

pub fn parse_args() -> CLIArgs {
    let matches = Command::new("passman")
        .version("0.1")
        .author("Yamil <issayamilrt@gmail.com>")
        .about("A simple local password manager")
        .arg(
            Arg::new("command")
                .help("Command : add, get, list, delete")
                .required(true),
        )
        .arg(
            Arg::new("service")
                .help("Service name (gmail, github...)")
                .required(false),
        )
        .get_matches();

    let command = matches
        .get_one::<String>("command")
        .unwrap()
        .to_string();

    let service = matches
        .get_one::<String>("service")
        .map(|s| s.to_string());

    CLIArgs { command, service }
}
