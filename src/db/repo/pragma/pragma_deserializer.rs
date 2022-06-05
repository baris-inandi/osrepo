use super::RepoPragma;

#[derive(Debug, serde::Deserialize)]
pub struct RepoPragmaDeserializer {
    pub name: String,
    pub description: Option<String>,
}

impl RepoPragmaDeserializer {
    pub fn to_pragma(&self, path: &str) -> RepoPragma {
        return RepoPragma {
            name: self.name.clone(),
            description: self.description.clone(),
            path: String::from(path),
        };
    }
}
