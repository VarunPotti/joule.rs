use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Debug)]
pub struct Package {
    package_name: String,
    display_name: String,
    version: String,
    threads: String,
    url: String,
    file_type: String,
    iswitches: Vec<String>,
    uswitches: Vec<String>,
    dependencies: Value,
}
impl Package {
    pub fn new(
        package_name: String,
        display_name: String,
        version: String,
        threads: String,
        url: String,
        file_type: String,
        iswitches: Vec<String>,
        uswitches: Vec<String>,
        dependencies: Value,
    ) -> Self {
        Self {
            package_name,
            display_name,
            version,
            threads,
            url,
            file_type,
            iswitches,
            uswitches,
            dependencies,
        }
    }
}
