use super::entry::entry_deserializer::EntryDeserializer;
use super::repo_pragma::RepoPragma;
use std::collections::HashMap;

#[derive(serde::Deserialize)]
pub struct DbDeserializer {
    pub include: Option<HashMap<RepoPragma, HashMap<String, EntryDeserializer>>>,
    pub update_url: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct MinimalDbDeserializer {
    pub update_url: Option<String>,
}
