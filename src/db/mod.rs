pub mod db_deserializer;
pub mod entry;
pub mod repo_pragma;
pub mod update;
use db_deserializer::DbDeserializer;
use entry::Entry;
use reqwest::Client;
use serde_yaml::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Db {
    pub includes: HashMap<String, String>,
    pub entries: HashMap<String, Entry>,
    pub client: Client,
    pub update_url: Option<String>,
    pub path: String,
}

impl Db {
    pub fn new(filepath: &str) -> Result<Db, serde_yaml::Error> {
        let file = crate::utils::expand_open_file(filepath).expect(&format!(
            "Cannot open db file '{}', does it exist?",
            &filepath
        ));
        let value: Value = serde_yaml::from_reader(file)?;
        let deserializer: DbDeserializer = serde_yaml::from_value(value)?;
        let mut includes: HashMap<String, String> = HashMap::new();
        let mut db_entries: HashMap<String, Entry> = HashMap::new();
        match deserializer.include {
            Some(i) => {
                for (repo_pragma, entries) in i {
                    includes.insert(repo_pragma.repo.clone(), repo_pragma.description);
                    for (identifier, entry_deserializer) in entries {
                        let entry = entry_deserializer.to_entry(&identifier, &repo_pragma.repo);
                        db_entries.insert(identifier, entry);
                    }
                }
            }
            None => {}
        }
        return Ok(Db {
            includes,
            entries: db_entries,
            client: Client::new(),
            update_url: deserializer.update_url,
            path: String::from(filepath),
        });
    }

    pub fn entry(&self, identifier: &str) -> Result<&Entry, std::io::Error> {
        return self.entries.get(identifier).ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Entry '{}' not found in database.", &identifier),
            )
        });
    }
}
