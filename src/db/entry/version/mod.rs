pub mod download;
pub mod version_deserializer;
use colored::Colorize;

#[derive(Debug)]
pub struct Version {
    pub parent_identifier: String,
    pub parent_repo: String,
    pub version_identifier: String,
    pub url: String,
    pub arch: String,
    pub browser: bool,
    pub is_prerelease: bool,
    pub ext: String,
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let is_prerelease = match self.is_prerelease {
            true => " [prerelease]".red(),
            false => "".white(),
        };
        let arch = format!("[{}]", self.arch).purple();
        let version_identifier = self.version_identifier.yellow();
        let parent_repo = self.parent_repo.blue().bold();
        return write!(
            f,
            "{}/{}@{} {}{}",
            parent_repo, self.parent_identifier, version_identifier, arch, is_prerelease
        );
    }
}
