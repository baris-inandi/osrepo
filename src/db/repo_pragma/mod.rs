#[derive(serde::Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RepoPragma {
    pub repo: String,
    pub description: String,
}
