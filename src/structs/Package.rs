use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Value;
use std::collections::HashMap; // 1.0.64
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct Package {
    pub package_name: String,
    pub display_name: String,
    pub version: String,
    pub threads: i8,
    pub url: String,
    pub file_type: String,
    pub iswitches: Vec<Value>,
    pub uswitches: Vec<Value>,
    pub dependencies: Vec<Value>,
    pub creator: String,
    pub Home_page: String,
}
