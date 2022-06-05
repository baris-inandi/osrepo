pub mod entry;
pub mod entry_deserializer;
use super::db_included_repo::DbIncludedRepo;
use super::repo_deserializer::RepoDeserializer;
use super::repo_pragma::RepoPragma;
use serde_yaml::Value;
use std::collections::HashMap;
use std::io::Read;

#[derive(Debug)]
pub struct Repo {
    pub pragma: RepoPragma,
    pub entries: HashMap<String, entry::Entry>,
}

impl Repo {
    pub fn new(filepath: &str) -> Option<Repo> {
        let mut file = crate::utils::expand_open_file(filepath).expect(&format!(
            "Cannot open repo \"{}\", does it exist?",
            filepath
        ));
        let mut repo_file_content = String::new();
        file.read_to_string(&mut repo_file_content).expect(&format!(
            "Cannot read file \"{}\", does it exist?",
            filepath
        ));
        let repo_file_content_trimmed = repo_file_content.trim();
        if repo_file_content_trimmed == "" {
            return None;
        }
        println!("{}", repo_file_content);
        let value: Value =
            serde_yaml::from_reader(file).expect(&format!("Cannot parse yaml \"{}\"", filepath));
        let repo_deserializer: RepoDeserializer = serde_yaml::from_value(value)
            .expect(&format!("Cannot deserialize repo \"{}\"", &filepath));
        let repo = repo_deserializer.to_repo(&repo_deserializer.pragma.name.clone());
        return Some(repo);
    }

    pub fn to_db_included_repo(&self, filepath: &str) -> DbIncludedRepo {
        return DbIncludedRepo {
            pragma: self.pragma.clone(),
            path: String::from(filepath),
        };
    }
}
