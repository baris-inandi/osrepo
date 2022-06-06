pub mod entry_deserializer;
pub mod version;
use colored::Colorize;
use std::collections::HashMap;
use version::Version;

#[derive(Debug)]

pub struct Entry {
    pub description: Option<String>,
    pub is_proprietary: bool,
    pub versions: HashMap<String, version::Version>,
    pub identifier: String,
    pub repo_name: String,
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let proprietary_text = match self.is_proprietary {
            true => "[proprietary] ".red(),
            false => "".white(),
        };
        let repo = self.repo_name.blue().bold();
        let description = match self.description {
            Some(ref description) => String::from(description),
            None => String::from("[no description]"),
        };
        let version_text;
        if self.versions.len() == 0 {
            version_text = String::from("[no versions provided]");
        } else if self.versions.len() == 1 {
            let versions: Vec<&String> = self.versions.keys().into_iter().collect();
            version_text = String::from(versions.first().unwrap().clone());
        } else {
            let versions = self.versions.keys().into_iter().len();
            version_text = format!("[{} versions available]", versions);
        };
        let version_text_color = version_text.green();
        return write!(
            f,
            "{}/{} {}\n{}{}",
            repo, self.identifier, version_text_color, proprietary_text, description
        );
    }
}

impl Entry {
    pub fn version(&self, version_identifier: &str) -> Result<&Version, std::io::Error> {
        return self.versions.get(version_identifier).ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!(
                    "Version '{}' not found in entry '{}'.",
                    &version_identifier, &self.identifier
                ),
            )
        });
    }
}
