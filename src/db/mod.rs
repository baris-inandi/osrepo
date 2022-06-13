pub mod db_deserializer;
pub mod search;
pub mod entry;
pub mod repo_pragma;
pub mod update;
use db_deserializer::DbDeserializer;
use entry::version::Version;
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
    pub backup_path: String,
}

impl Db {
    pub fn new(filepath: &str) -> Result<Db, serde_yaml::Error> {
        let file = crate::utils::fs::expand_open_file(filepath).expect(&format!(
            "Cannot open db file '{}', does it exist?",
            &filepath
        ));
        let value: Value = serde_yaml::from_reader(file)?;
        let path = String::from(filepath);
        let deserializer: DbDeserializer = serde_yaml::from_value(value)?;
        let mut includes: HashMap<String, String> = HashMap::new();
        let mut db_entries: HashMap<String, Entry> = HashMap::new();
        match deserializer.include {
            Some(i) => {
                for (repo_pragma, entries) in i {
                    includes.insert(repo_pragma.repo.clone(), repo_pragma.description);
                    for (k, v) in entries {
                        let mut current_entry_versions: HashMap<String, Version> = HashMap::new();
                        for (version_identifier, vd) in v {
                            let version =
                                vd.to_version(&version_identifier, &k.entry, &repo_pragma.repo);
                            current_entry_versions.insert(version_identifier, version);
                        }
                        let entry = k.to_entry(&repo_pragma.repo, current_entry_versions);
                        db_entries.insert(k.entry, entry);
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
            path: path.clone(),
            backup_path: format!("{}.bak", &path),
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

    pub fn _print(&self) {
        println!("{:?}", self);
    }
}
