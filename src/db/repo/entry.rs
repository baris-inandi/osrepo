pub mod version;
pub mod version_deserializer;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Entry {
    pub description: Option<String>,
    pub is_proprietary: bool,
    pub versions: HashMap<String, version::Version>,
    pub identifier: String,
    pub repo_name: String,
}

/*
    impl getVersions etc.
*/
