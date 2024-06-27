use clap::{command, Arg, Command};

fn main() {
    let m = command!()
        .subcommand(
            Command::new("create")
                .about("This command create a new user")
                .arg(
                    Arg::new("username")
                        .short('u')
                        .long("username")
                        .aliases(["uname"])
                        .required(true)
                        .help("Username"), //.conflicts_with("email"),
                )
                .arg(
                    Arg::new("email")
                        .short('e')
                        .long("email")
                        .help("Email address"),
                ),
        )
        .about("Application is unknown")
        .version("1.0.0")
        .get_matches();

    if let Some(create) = m.subcommand_matches("create") {
        println!(
            "Username: {} and Email address: {}",
            create.get_one::<String>("username").unwrap(),
            create
                .get_one::<String>("email")
                .unwrap_or(&"No email provided".to_string())
        )
    }
}
