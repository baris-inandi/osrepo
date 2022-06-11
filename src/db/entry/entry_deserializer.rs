use super::version::version_deserializer::VersionDeserializer;
use super::Entry;
use std::collections::HashMap;

#[derive(serde::Deserialize, Debug)]
pub struct EntryDeserializer {
    description: Option<String>,
    proprietary: Option<bool>,
    versions: Option<HashMap<String, VersionDeserializer>>,
}

impl EntryDeserializer {
    pub fn to_entry(&self, identifier: &str, repo_name: &str) -> Entry {
        let allowed_chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz1234567890-".chars().collect();
        for i in identifier.clone().chars() {
            let allowed = allowed_chars.contains(&i);
            allowed.then(|| true)
            .expect(
                &format!("Name of entry '{}' in repo '{}' is invalid: Entry names can be composed of lowercase English letters, numbers and hyphens only", identifier, repo_name)
            );
        }
        return Entry {
            description: self.description.clone(),
            is_proprietary: match self.proprietary {
                Some(proprietary) => proprietary,
                None => false,
            },
            versions: match &self.versions {
                Some(versions) => versions
                    .iter()
                    .map(|(k, v)| (k.clone(), v.to_version(k, &identifier, &repo_name)))
                    .collect(),
                None => HashMap::new(),
            },
            identifier: String::from(identifier),
            repo_name: String::from(repo_name),
        };
    }
}
