use super::entry::entry_pragma::EntryPragma;
use super::entry::version::version_deserializer::VersionDeserializer;
use super::repo_pragma::RepoPragma;
use std::collections::HashMap;

#[derive(serde::Deserialize)]
pub struct DbDeserializer {
    pub include:
        Option<HashMap<RepoPragma, HashMap<EntryPragma, HashMap<String, VersionDeserializer>>>>,
    pub update_url: Option<String>,
}
