mod version;
use version::Version;

pub struct Entry {
    identifier: String,
    description: String,
    is_proprietary: bool,
    versions: Vec<Version>,
}
