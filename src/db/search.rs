use super::entry::Entry;
use super::Db;
use itertools::Itertools;
use std::collections::HashMap;

fn similarity(key: &str, keyword: &str) -> f64 {
    let keyword_lower = keyword.to_lowercase();
    let key_lower = key.to_lowercase();
    let similarity: f64;
    if key_lower.contains(&keyword_lower) {
        similarity = 0.999;
    } else if keyword_lower.contains(&key_lower) {
        similarity = 0.998;
    } else {
        similarity = strsim::jaro(&key_lower, &keyword_lower);
    }
    let sim_rounded = f64::trunc(similarity * 1000.0) / 1000.0;
    return sim_rounded;
}

impl Db {
    fn display_indexed(&self, i: &HashMap<u64, &Entry>) {}

    pub fn search(&self, keyword: &str) -> Result<HashMap<u64, &Entry>, std::io::Error> {
        if keyword.len() < 1 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Search keyword is empty",
            ));
        }
        let keys = self.entries.keys();
        let first_letter = keyword.chars().next().unwrap();
        let treshold = 0.7;
        let mut keys_first_letter: Vec<&str> = Vec::new();
        for key in keys {
            if (&key).starts_with(first_letter) {
                keys_first_letter.push(&key);
            }
        }
        let mut out: HashMap<u64, &str> = HashMap::new();
        for key in keys_first_letter {
            // make both lowercase
            // pass if keyword is a substring of key
            let sim_rounded = similarity(&key, &keyword);
            if sim_rounded > treshold {
                let sim_whole = (sim_rounded * 1000.0).round() as u64;
                out.insert(sim_whole, key);
            }
        }
        if out.keys().len() < 1 {
            for key in self.entries.keys() {
                let sim_rounded = similarity(&key, &keyword);
                if sim_rounded > treshold {
                    let sim_whole = (sim_rounded * 1000.0).round() as u64;
                    out.insert(sim_whole, key);
                }
            }
        }
        let out_sorted_unlimited = out.values().sorted();
        let mut out_indexed: HashMap<u64, &Entry> = HashMap::new();
        for (idx, key) in out_sorted_unlimited.enumerate() {
            let entry = self.entry(key).unwrap();
            out_indexed.insert(idx as u64 + 1, &entry);
            if idx >= 100 {
                break;
            }
        }
        self.display_indexed(&out_indexed);
        return Ok(out_indexed);
    }
}
