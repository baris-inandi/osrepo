use super::Version;

#[derive(serde::Deserialize)]
pub struct VersionDeserializer {
    url: String,
    arch: Option<String>,
    is_prerelease: Option<bool>,
}

impl VersionDeserializer {
    pub fn to_version(
        &self,
        version_identifier: &str,
        parent_identifier: &str,
        parent_repo: &str,
    ) -> Version {
        let allowed_chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz1234567890_-.^=<>()"
            .chars()
            .collect();
        for i in version_identifier.clone().chars() {
            let allowed = allowed_chars.contains(&i);
            allowed.then(|| true)
            .expect(
                &format!("Version identifier '{}' is invalid: Version identifiers can be composed of lowercase English letters, numbers and the following symbols: _-.^=<>()", version_identifier)
            );
        }
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
            parent_identifier: String::from(parent_identifier),
            parent_repo: String::from(parent_repo),
        };
    }
}
