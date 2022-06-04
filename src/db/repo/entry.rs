mod version;
use std::collections::HashMap;
use version::Version;

#[derive(serde::Deserialize, Debug)]
pub struct Entry {
    description: Option<String>,
    is_proprietary: Option<bool>,
    versions: HashMap<String, Version>,
}

/*
    impl getVersions etc.
*/
