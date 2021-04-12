//? This file is responsible for the installation of a particular app
#[path = "../structs/Package.rs"]
mod package;
#[path = "../utils/requests.rs"]
mod requests;
pub fn install(app_name: &str) {
    let resp = requests::get_package(app_name);
    // let package = package::Package {
    //     package_name: resp["package_name"].to_string(),
    //     display_name: resp["display_name"].to_string(),
    //     version: resp["version"].to_string(),
    //     threads: resp["threads"].to_string(),
    //     url: resp[resp["version"]]["url"].to_string(),
    //     file_type: val,
    //     iswitches: val,
    //     uswitches: val,
    //     dependencies: val,
    // };
    let version = resp["version"].to_string();
    print!("{}", version);
    // println!("{}", resp[version]);
}
