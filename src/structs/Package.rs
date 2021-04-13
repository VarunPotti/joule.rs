use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Debug)]
pub struct Package {
    pub package_name: String,
    pub display_name: String,
    pub version: String,
    pub threads: String,
    pub url: String,
    pub file_type: String,
    pub iswitches: Vec<Value>,
    pub uswitches: Vec<Value>,
    pub dependencies: Vec<Value>,
}
