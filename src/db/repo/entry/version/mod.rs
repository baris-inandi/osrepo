pub mod version_deserializer;
use colored::Colorize;

#[derive(Debug)]
pub struct Version {
    pub parent_identifier: String,
    pub parent_repo: String,
    pub version_identifier: String,
    pub url: String,
    pub arch: String,
    pub is_prerelease: bool,
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let is_prerelease = match self.is_prerelease {
            true => " [pre]".red(),
            false => "".white(),
        };
        let arch = self.arch.purple();
        let version_identifier = self.version_identifier.yellow();
        let parent_repo = self.parent_repo.blue().bold();
        return write!(
            f,
            "{}/{}@{} {}{}",
            parent_repo, self.parent_identifier, version_identifier, arch, is_prerelease
        );
    }
}
