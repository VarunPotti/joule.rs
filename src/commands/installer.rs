//? This file is responsible for the installation of a particular app
#[path = "../structs/Package.rs"]
mod package;
#[path = "../utils/requests.rs"]
mod requests;

#[path = "../utils/ansi.rs"]
mod ansi;

use miniserde::{json, Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use std::ops::Deref;

#[derive(Deserialize, Serialize)]
pub struct PackageTemp {
    package_name: String,
    display_name: String,
    version: String,
}
#[derive(Deserialize, Serialize)]
struct PackageList {
    packages: Vec<String>,
}
pub async fn install(app_name: &str) -> miniserde::Result<()> {
    let mut s = String::new();

    let db_list = &format!(
        r#"{}\{}\{}"#,
        env::var("USERPROFILE").unwrap(),
        r#"AppData\Roaming"#,
        r#"Joule\data\dblist.json"#
    );
    let _contents = File::open(&db_list)
        .expect("Something went wrong reading the file")
        .read_to_string(&mut s);

    let json: PackageList = json::from_str(&s).unwrap();
    let package = json
        .packages
        .iter()
        .map(|item| item.deref())
        .collect::<Vec<&str>>();
    let result = difflib::get_close_matches(app_name, package, 1, 0.6);
    if result.len() > 0 {
        if app_name == result[0] {
            let resp: package::Package = json::from_str(&requests::get_package(app_name))?;
            println!("{:?}", resp);
            Ok(())
        } else {
            println!(
                "{}- {}",
                ansi::green("Autocorrecting to "),
                ansi::blue(result[0])
            );
            println!("Continue? (y/n)");

            let mut cont = String::new();
            io::stdin()
                .read_line(&mut cont)
                .expect("Failed to read line");
            if cont.to_ascii_lowercase() == "y\r\n" {
                let resp: package::Package = json::from_str(&requests::get_package(app_name))?;
                println!("{:?}", resp);
                Ok(())
            } else {
                println!("Exiting");
                Ok(())
            }
        }
    } else {
        println!(
            "{} {} {}\n\t {}",
            ansi::red("Couldn't find"),
            ansi::red(app_name),
            ansi::red(&format!(
                ":Run \"joule search {}\" to find apps matching {}",
                app_name, app_name
            )),
            ansi::blue("or ask for a package here https://github.com/joule-package-manager/joule-packages/issues/1")
        );
        Ok(())
    }
}
