use super::Version;

#[derive(serde::Deserialize)]
pub struct VersionDeserializer {
    url: String,
    arch: Option<String>,
    is_prerelease: Option<bool>,
}

impl VersionDeserializer {
    pub fn to_version(&self, version_identifier: &str) -> Version {
        let arch_option = self.arch.clone();
        return Version {
            version_identifier: String::from(version_identifier),
            url: self.url.clone(),
            arch: match arch_option {
                Some(arch) => arch,
                None => String::from("x86_64"),
            },
            is_prerelease: match self.is_prerelease {
                Some(is_prerelease) => is_prerelease,
                None => false,
            },
        };
    }
}
