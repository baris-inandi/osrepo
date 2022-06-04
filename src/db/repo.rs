mod entry;
use entry::Entry;

pub struct Repo {
    name: String,
    entries: Vec<Entry>,
}

impl Repo {
    pub fn new(filepath: &str) -> Repo {
        println!("Loading repo from {}", filepath);
        return Repo {
            name: String::new(),
            entries: Vec::new(),
        };
    }
}
