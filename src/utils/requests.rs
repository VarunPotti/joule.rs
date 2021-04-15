#[path = "./ansi.rs"]
mod ansi;
use serde_json::{ Value, from_str };
use std::process;
use minreq;

#[allow(unused)]
pub fn get_package(name: &str) -> Value {
    let mut response: String = String::new();

    match minreq::get(format!("https://raw.githubusercontent.com/joule-package-manager/joule-packages/main/packages/{}.json", name)).send() {
        Ok(text) => {
            let res = text.as_str().unwrap();
            if res != "404: Not Found" {
                response = res.to_string();            
            }
        },
        Err(error) => {
            println!("{} => {}", ansi::red(format!("Could not find {}", ansi::blue(name)).as_str()), error);
        },
    }
    
    match from_str(response.as_str()) {
        Ok(data) => {
            data
        },
        Err(error) => {
            println!("{} : {}", ansi::red("An Error Occured While Parsing JSON File"), ansi::yellow(error.to_string().as_str()));
            process::exit(1);
        }
    }
}
#[allow(unused)]
pub fn download_file(url: &str) -> String{
    let mut response: String = String::new();
    
    match minreq::get(url).send() {
        Ok(text) => {
            let res = text.as_str().unwrap();
            if res != "404: Not Found" {
                response = res.to_string();            
            }
        },
        Err(error) => {
            println!("{}", ansi::red(error.to_string().as_str()))
        },
    }
    response
}
