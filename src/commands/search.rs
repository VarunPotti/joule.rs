#[path = "../utils/ansi.rs"]
mod ansi;

#[path = "../utils/cacher.rs"]
mod cacher;

use miniserde::{json, Deserialize, Serialize};

//? Imports
use std::env;
use std::fs::File;
use std::io::Read;
use std::ops::Deref;

#[derive(Deserialize, Serialize)]
struct PackageList {
    packages: Vec<String>,
}

pub fn search(app_name: &str, resultcount: &std::string::String) {
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
    let result = difflib::get_close_matches(
        app_name,
        package,
        resultcount.parse::<usize>().unwrap(),
        0.5,
    );
    if result.len() > 0 {
        println!("{}{}\n", ansi::blue("Result for: "), ansi::green(app_name));
    } else {
        println!(
            "{} {}",
            ansi::red("Found no results for"),
            ansi::red(app_name)
        )
    }
    for res in result {
        println!(
            "{} \t: Install command {} {}",
            ansi::blue(res),
            ansi::blue("joule install"),
            ansi::blue(res)
        );
    }
}
