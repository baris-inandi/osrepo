pub mod pragma_deserializer;

#[derive(Debug, Clone)]
pub struct RepoPragma {
    pub name: String,
    pub description: Option<String>,
    pub path: String,
}
