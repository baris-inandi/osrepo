pub mod entry;
pub mod repo_deserializer;
use repo_deserializer::RepoDeserializer;
pub mod pragma;
use pragma::RepoPragma;
use serde_yaml::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Repo {
    pub pragma: RepoPragma,
    pub entries: HashMap<String, entry::Entry>,
}

impl Repo {
    pub fn new(filepath: &str) -> Option<Repo> {
        let file = crate::utils::expand_open_file(filepath)
            .expect(&format!("Cannot open repo '{}', does it exist?", filepath));
        let value: Value =
            serde_yaml::from_reader(file).expect(&format!("Cannot parse yaml '{}'", filepath));
        let repo_deserializer: RepoDeserializer = serde_yaml::from_value(value)
            .expect(&format!("Cannot deserialize repo '{}'", &filepath));
        let repo = repo_deserializer.to_repo(&repo_deserializer.pragma.name.clone(), &filepath);
        return Some(repo);
    }
}
