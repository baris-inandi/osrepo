pub mod db_included_repo;
pub mod repo;
pub mod repo_deserializer;
pub mod repo_pragma;
use db_included_repo::DbIncludedRepo;
use repo::entry::Entry;
use serde_yaml::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Db {
    pub includes: HashMap<String, DbIncludedRepo>,
    pub entries: HashMap<String, Entry>,
}

impl Db {
    pub fn new(filepath: &str) -> Result<Db, serde_yaml::Error> {
        let file = crate::utils::expand_open_file(filepath).expect(&format!(
            "Cannot open db file \"{}\", does it exist?",
            &filepath
        ));
        let value: Value = serde_yaml::from_reader(file)?;
        let repos_value = value
            .get("include")
            .expect(&format!("Database file at \"{}\" must contain an include key", &filepath))
            .as_sequence()
            .expect(
                format!(
                    "Database file at \"{}\" must include a list of repositories under key \"include\"",
                    &filepath
                )
                .as_str(),
            )
            .to_vec();
        let mut includes: HashMap<String, DbIncludedRepo> = HashMap::new();
        let mut db_entries: HashMap<String, Entry> = HashMap::new();
        for (idx, p) in repos_value.iter().enumerate() {
            let path = p.as_str().expect(&format!(
                "Database file at \"{}\": key \"include\" does not contain a string at index {}.",
                &filepath, &idx
            ));
            let current_repo = match repo::Repo::new(path) {
                Some(repo) => repo,
                None => continue,
            };
            let db_included_repo = current_repo.to_db_included_repo(path);
            db_entries.extend(current_repo.entries);
            includes.insert(current_repo.pragma.name.clone(), db_included_repo);
        }
        return Ok(Db {
            includes,
            entries: db_entries,
        });
    }
}
