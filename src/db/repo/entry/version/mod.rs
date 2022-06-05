pub mod version_deserializer;

#[derive(Debug)]
pub struct Version {
    pub version_identifier: String,
    pub url: String,
    pub arch: String,
    pub is_prerelease: bool,
}
