use super::Version;

#[derive(serde::Deserialize, Debug)]
pub struct VersionDeserializer {
    url: String,              // download url of the version
    arch: Option<String>,     // architecture of the target image (x86_64, arm)
    prerelease: Option<bool>, // notify the user that this version is a prerelease
    browser: Option<bool>, // option to open url in browser, useful for dynamically generated urls (eg. windows)
    ext: Option<String>,   // file extension for file download (iso, img)
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
        return Version {
            version_identifier: String::from(version_identifier),
            url: self.url.clone(),
            arch: match &self.arch {
                Some(arch) => String::from(arch),
                None => String::from("x86_64"),
            },
            ext: match &self.ext {
                Some(ext) => String::from(ext),
                None => String::from("iso"),
            },
            is_prerelease: match &self.prerelease {
                Some(prerelease) => prerelease.to_owned(),
                None => false,
            },
            browser: match &self.browser {
                Some(browser) => browser.to_owned(),
                None => false,
            },
            parent_identifier: String::from(parent_identifier),
            parent_repo: String::from(parent_repo),
        };
    }
}
