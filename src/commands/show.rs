//? This file is responsible for the installation of a particular app
#[path = "../utils/requests.rs"]
mod requests;

#[path = "../utils/ansi.rs"]
mod ansi;

#[path = "../structs/Package.rs"]
mod package;

use serde::{Deserialize, Serialize};
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
#[derive(serde::Deserialize, serde::Serialize)]
struct PackageList {
    packages: Vec<String>,
}
fn print(pkt: package::Package) {
    println!("Name: {}", ansi::blue(&pkt.display_name));
    println!("Creator: {}", ansi::blue(&pkt.creator));
    println!("HomePage: {}", ansi::blue(&pkt.Home_page));
    println!("Version: {}", ansi::blue(&pkt.version));
    println!("Threads: {}", ansi::blue(&pkt.threads.to_string()));
    println!("Download Url: {}", ansi::blue(&pkt.url));
    println!("File_type: {}", ansi::blue(&pkt.file_type));
    println!("Install Switches: ");
    for p in pkt.iswitches {
        println!("\t{}", ansi::blue(&p.to_string()));
    }
    println!("Install Switches: ");
    for p in pkt.uswitches {
        println!("\t{}", ansi::blue(&p.to_string()));
    }

    println!("Dependencies:");
    if pkt.dependencies.len() == 0 {
        println!("\t {}", ansi::blue("This package has no dependencies"));
    } else {
        for p in pkt.dependencies {
            println!("\t{}", ansi::blue(&p.to_string()));
        }
    }
}
pub fn show(app_name: &str) {
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

    let json: PackageList = serde_json::from_str(&s).unwrap();
    let package = json
        .packages
        .iter()
        .map(|item| item.deref())
        .collect::<Vec<&str>>();
    let result = difflib::get_close_matches(app_name, package, 1, 0.6);
    if app_name == result[0] {
        let resp = requests::get_package(app_name);
        let ref pkg = resp[&resp[r#"version"#].to_string()];
        let _package = package::Package {
            package_name: resp["package_name"].to_string(),
            display_name: resp["display_name"].to_string(),
            version: resp["version"].to_string(),
            threads: resp["threads"].to_string().parse::<i8>().unwrap(),
            url: pkg["url"].to_string(),
            file_type: pkg[r#"file-type"#].to_string(),
            iswitches: pkg["iswitches"].as_array().unwrap().to_vec(),
            uswitches: pkg["uswitches"].as_array().unwrap().to_vec(),
            dependencies: pkg["dependencies"].as_array().unwrap().to_vec(),
            Home_page: resp["Home_page"].to_string(),
            creator: resp["creator"].to_string(),
        };
        print(_package);
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
            let resp = requests::get_package(app_name);
            let ref pkg = resp[&resp[r#"version"#].to_string()];
            let _package = package::Package {
                package_name: resp["package_name"].to_string(),
                display_name: resp["display_name"].to_string(),
                version: resp["version"].to_string(),
                threads: resp["threads"].to_string().parse::<i8>().unwrap(),
                url: pkg["url"].to_string(),
                file_type: pkg[r#"file-type"#].to_string(),
                iswitches: pkg["iswitches"].as_array().unwrap().to_vec(),
                uswitches: pkg["uswitches"].as_array().unwrap().to_vec(),
                dependencies: pkg["dependencies"].as_array().unwrap().to_vec(),
                Home_page: resp["Home_page"].to_string(),
                creator: resp["creator"].to_string(),
            };

            println!("{:?}", _package.iswitches);
            print(_package);
        } else {
            println!("Exiting")
        }
    }
}
