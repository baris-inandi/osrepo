mod entry;
use entry::Entry;
use serde_yaml::Value;
use std::collections::HashMap;

#[derive(serde::Deserialize)]
struct RepoMeta {
    name: String,
    description: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct Repo {
    pragma: RepoMeta,
    entries: Option<HashMap<String, Entry>>,
}

impl Repo {
    pub fn new(filepath: &str) -> Repo {
        let file = crate::utils::expand_open_file(filepath).unwrap();
        let value: Value = serde_yaml::from_reader(file).unwrap();
        return serde_yaml::from_value(value).unwrap();
    }
}
