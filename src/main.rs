#[path = "./utils/ansi.rs"]
mod ansi;

#[path = "./commands/installer.rs"]
mod installer;

#[path = "./commands/search.rs"]
mod search;

#[path = "./utils/cacher.rs"]
mod cacher;

//? Imports
use std::env;
use std::fs;

//?Global Vars
const __VERSION__: &str = env!("CARGO_PKG_VERSION");

//?Functions
#[allow(unused_must_use)]
fn create_dirs() {
    let dir = format!(
        r#"{}\{}\{}"#,
        env::var("USERPROFILE").unwrap(),
        r#"AppData\Roaming"#,
        "Joule"
    );
    fs::create_dir_all(format!("{}", dir));
    fs::create_dir_all(format!(r#"{}\{}"#, dir, "data"));
}

fn main() {
    //? Enable Coloured Terminal -------------
    ansi_term::enable_ansi_support().unwrap();
    //? Create Required Directories

    create_dirs();
    //? Cache files
    cacher::cache();
    let arguments: Vec<String> = env::args().collect();

    //? Checks if the length of a arguments(var) is more than 2 if yes do the operation else show help for the command
    if arguments.len() > 2 {
        let command: &str = &arguments[1];
        if command == "install" {
            let _ = installer::install(&arguments[2]);
        } else if command == "search" {
            if arguments.len() == 4 {
                let _ = search::search(&arguments[2], &arguments[3]);
            } else {
                let _ = search::search(&arguments[2], &"1".to_string());
            }
        }
    } else if arguments.len() == 2 {
        let command: &str = &arguments[1];

        match command {
            "install" => println!(
                "{}",
                format!(
                    "{} \n  {}{}",
                    format!("Joule {}{}", ansi::green("v"), ansi::green(__VERSION__)),
                    ansi::blue("install <App>: "),
                    ansi::green("Installs An App")
                )
            ),
            "search" => println!(
                "{}",
                format!(
                    "{} \n  {}: {}",
                    format!("Joule {}{}", ansi::green("v"), ansi::green(__VERSION__)),
                    ansi::blue("search <name> <optional:resultcount>"),
                    ansi::blue(
                        "Searches an app. NOTE: \"resultcount\" is optional(max value is 255)"
                    )
                )
            ),
            _ => {}
        }
    } else {
        let help = format!(
            r#"{}
{} {} - Add a package
{} {} - Uninstall a package
{} {} - Show a package
{} {} - Search for a package"#,
            format!("Joule {}{}", ansi::green("v"), ansi::green(__VERSION__)),
            ansi::purple("*"),
            ansi::blue("install"),
            ansi::purple("*"),
            ansi::blue("uninstall"),
            ansi::purple("*"),
            ansi::blue("show"),
            ansi::purple("*"),
            ansi::blue("search")
        );
        println!("{}", help);
    }
}
