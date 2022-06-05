pub mod entry_deserializer;
pub mod version;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Entry {
    pub description: Option<String>,
    pub is_proprietary: bool,
    pub versions: HashMap<String, version::Version>,
    pub identifier: String,
    pub repo_name: String,
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let out = String::from("");
        return out.fmt(f);
    }
}
