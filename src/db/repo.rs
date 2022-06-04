mod entry;
use entry::Entry;
use serde_yaml::Value;
use std::collections::HashMap;

#[derive(serde::Deserialize, Debug)]
struct RepoMeta {
    name: String,
    description: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Repo {
    meta: RepoMeta,
    entries: Option<HashMap<String, Entry>>,
}

impl Repo {
    pub fn new(filepath: &str) -> Repo {
        println!("Loading repo from {}", filepath);
        let file = crate::utils::expand_open_file(filepath).unwrap();
        let value: Value = serde_yaml::from_reader(file).unwrap();
        return serde_yaml::from_value(value).unwrap();
    }
}
