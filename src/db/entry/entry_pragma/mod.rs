use super::version::Version;
use super::Entry;
use std::collections::HashMap;

#[derive(serde::Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EntryPragma {
    pub entry: String,
    pub description: Option<String>,
    pub proprietary: Option<bool>,
}

impl EntryPragma {
    pub fn to_entry(&self, repo_name: &str, versions: HashMap<String, Version>) -> Entry {
        return Entry {
            identifier: String::from(&self.entry),
            description: self.description.clone(),
            is_proprietary: match self.proprietary {
                Some(p) => p,
                None => false,
            },
            repo_name: String::from(repo_name),
            versions,
        };
    }
}
