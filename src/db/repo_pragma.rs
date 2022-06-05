#[derive(Debug, serde::Deserialize, Clone)]
pub struct RepoPragma {
    pub name: String,
    pub description: Option<String>,
}
