use super::entry::entry_deserializer::EntryDeserializer;
use super::pragma::pragma_deserializer::RepoPragmaDeserializer;
use super::Repo;
use std::collections::HashMap;

#[derive(serde::Deserialize)]
pub struct RepoDeserializer {
    pub pragma: RepoPragmaDeserializer,
    entries: Option<HashMap<String, EntryDeserializer>>,
}

impl RepoDeserializer {
    pub fn to_repo(&self, repo_name: &str, repo_path: &str) -> Repo {
        let entries_option = self.entries.as_ref().map(|entries| {
            entries
                .iter()
                .map(|(k, v)| (k.clone(), v.to_entry(k, repo_name)))
                .collect()
        });
        return Repo {
            pragma: self.pragma.to_pragma(repo_path),
            entries: match entries_option {
                Some(entries) => entries,
                None => HashMap::new(),
            },
        };
    }
}
