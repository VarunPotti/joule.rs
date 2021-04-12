#[path = "./utils/ansi.rs"]
mod ansi;

#[path = "./commands/installer.rs"]
mod installer;

use std::env;

fn main() {
    //? Enable Coloured Terminal -------------
    ansi_term::enable_ansi_support().unwrap();

    let arguments: Vec<String> = env::args().collect();

    //? Checks if the length of a arguments(var) is more than 2 if yes do the operation else show help for the command
    if arguments.len() > 2 {
        let command: &str = &arguments[1];
        if command == "install" {
            let _ = installer::install(&arguments[2]);
        }
    } else if arguments.len() == 2 {
        let command: &str = &arguments[1];

        match command {
            "install" => println!(
                "{}",
                format!(
                    "{} \n  {}{}",
                    ansi::green("Joule Package Manager"),
                    ansi::blue("install <App>: "),
                    ansi::green("Installs An App")
                )
            ),
            _ => {}
        }
    }
}
