pub mod repo;
use repo::entry::Entry;
use repo::pragma::RepoPragma;
use serde_yaml::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Db {
    pub includes: HashMap<String, RepoPragma>,
    pub entries: HashMap<String, Entry>,
}

impl Db {
    pub fn new(filepath: &str) -> Result<Db, serde_yaml::Error> {
        let file = crate::utils::expand_open_file(filepath).expect(&format!(
            "Cannot open db file '{}', does it exist?",
            &filepath
        ));
        let value: Value = serde_yaml::from_reader(file)?;
        let repos_value = value
            .get("include")
            .expect(&format!(
                "Database file at '{}' must contain an include key",
                &filepath
            ))
            .as_sequence()
            .expect(
                format!(
                    "Database file at '{}' must include a list of repositories under key 'include'",
                    &filepath
                )
                .as_str(),
            )
            .to_vec();
        let mut includes: HashMap<String, RepoPragma> = HashMap::new();
        let mut db_entries: HashMap<String, Entry> = HashMap::new();
        for (idx, p) in repos_value.iter().enumerate() {
            let path = p.as_str().expect(&format!(
                "Database file at '{}': key 'include' does not contain a string at index {}.",
                &filepath, &idx
            ));
            let current_repo = match repo::Repo::new(path) {
                Some(repo) => repo,
                None => continue,
            };
            db_entries.extend(current_repo.entries);
            includes.insert(current_repo.pragma.name.clone(), current_repo.pragma);
        }
        return Ok(Db {
            includes,
            entries: db_entries,
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
