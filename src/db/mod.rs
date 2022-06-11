pub mod db_deserializer;
pub mod entry;
pub mod repo_pragma;
pub mod update;
use db_deserializer::{DbDeserializer, MinimalDbDeserializer};
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
    pub serde_value: serde_yaml::Value,
    pub is_entries_loaded: bool,
}

impl Db {
    pub fn new(filepath: &str) -> Result<Db, serde_yaml::Error> {
        let file = crate::utils::fs::expand_open_file(filepath).expect(&format!(
            "Cannot open db file '{}', does it exist?",
            &filepath
        ));
        let value: Value = serde_yaml::from_reader(file)?;
        let minimal: MinimalDbDeserializer = serde_yaml::from_value(value.clone())?;
        return Ok(Db {
            includes: HashMap::new(),
            entries: HashMap::new(),
            client: Client::new(),
            update_url: minimal.update_url,
            path: String::from(filepath),
            serde_value: value,
            is_entries_loaded: false,
        });
    }

    pub fn load_entries(&mut self) -> Result<(), serde_yaml::Error> {
        let deserializer: DbDeserializer = serde_yaml::from_value(self.serde_value.clone())?;
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
        self.includes = includes;
        self.entries = db_entries;
        self.is_entries_loaded = true;
        return Ok(());
    }

    pub fn entry(&mut self, identifier: &str) -> Result<&Entry, std::io::Error> {
        if !&self.is_entries_loaded {
            self.load_entries()
                .expect("Failed to load database entries");
        }
        return self.entries.get(identifier).ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Entry '{}' not found in database.", &identifier),
            )
        });
    }
}
