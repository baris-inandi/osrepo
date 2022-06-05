use super::repo_pragma::RepoPragma;

#[derive(Debug)]
pub struct DbIncludedRepo {
    pub pragma: RepoPragma,
    pub path: String,
}
