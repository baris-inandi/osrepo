use super::version::version_deserializer::VersionDeserializer;
use super::Entry;
use std::collections::HashMap;

#[derive(serde::Deserialize)]
pub struct EntryDeserializer {
    description: Option<String>,
    is_proprietary: Option<bool>,
    versions: Option<HashMap<String, VersionDeserializer>>,
}

impl EntryDeserializer {
    pub fn to_entry(&self, identifier: &str, repo_name: &str) -> Entry {
        let allowed_chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz1234567890-".chars().collect();
        for i in identifier.clone().chars() {
            let allowed = allowed_chars.contains(&i);
            allowed.then(|| true)
            .expect(
                &format!("Name of entry \"{}\" in repo \"{}\" is invalid: Entry names should be composed of English letters, numbers and hyphens only", identifier, repo_name)
            );
        }
        return Entry {
            description: self.description.clone(),
            is_proprietary: match self.is_proprietary {
                Some(is_proprietary) => is_proprietary,
                None => false,
            },
            versions: match &self.versions {
                Some(versions) => versions
                    .iter()
                    .map(|(k, v)| (k.clone(), v.to_version(k)))
                    .collect(),
                None => HashMap::new(),
            },
            identifier: String::from(identifier),
            repo_name: String::from(repo_name),
        };
    }
}
