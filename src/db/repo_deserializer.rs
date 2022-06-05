use super::repo::entry_deserializer::EntryDeserializer;
use super::repo::Repo;
use super::repo_pragma::RepoPragma;
use std::collections::HashMap;

#[derive(serde::Deserialize)]
pub struct RepoDeserializer {
    pub pragma: RepoPragma,
    entries: Option<HashMap<String, EntryDeserializer>>,
}

impl RepoDeserializer {
    pub fn to_repo(&self, repo_name: &str) -> Repo {
        let entries_option = self.entries.as_ref().map(|entries| {
            entries
                .iter()
                .map(|(k, v)| (k.clone(), v.to_entry(k, repo_name)))
                .collect()
        });
        return Repo {
            pragma: RepoPragma {
                name: self.pragma.name.clone(),
                description: self.pragma.description.clone(),
            },
            entries: match entries_option {
                Some(entries) => entries,
                None => HashMap::new(),
            },
        };
    }
}
