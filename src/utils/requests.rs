#[path = "./ansi.rs"]
mod ansi;
use reqwest::blocking::get;
use serde_json::{from_str, Value};
use std::process;

#[allow(unused)]
pub fn get_package(name: &str) -> Value {
    let mut data = String::new();

    match get(format!("https://raw.githubusercontent.com/joule-package-manager/joule-packages/main/packages/{}.json", name)) {
        Ok(response) => {
            if response.status() == reqwest::StatusCode::OK {
                match response.text() {
                    Ok(text) => {
                        data = text;
                    }
                    Err(err) => eprintln!("Could Not Read Response JSON, {}", err),
                }
            } else {
                println!("Response Was Not 200 OK");
            }
        }
        Err(err) => eprintln!("Failed To Send Request: {}", err),
    }

    let res = from_str(&data).unwrap_or_else(|error| {
        // Display Error Message And Exit
        eprintln!("An Error Occured While Parsing The Json Data : {}", error);
        process::exit(1);
    });

    res
}

#[allow(unused)]
pub fn download_file(url: &str) -> String {
    let mut data = String::new();

    match get(url) {
        Ok(response) => {
            // Check if 200 OK
            if response.status() == reqwest::StatusCode::OK {
                match response.text() {
                    Ok(text) => {
                        data = text;
                    }
                    Err(err) => eprintln!("Could Not Read Response JSON, {}", err),
                }
            } else {
                println!("Response Was Not 200 OK");
            }
        }
        Err(err) => eprintln!("Failed To Send Request: {}", err),
    }

    data
}
