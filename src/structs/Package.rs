use miniserde::{json, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    pub package_name: String,
    pub display_name: String,
    pub version: String,
    pub threads: String,
    pub url: String,
    pub file_type: String,
    pub iswitches: Vec<String>,
    pub uswitches: Vec<String>,
    pub dependencies: Vec<String>,
    pub creator: String,
    pub Home_page: String,
}
