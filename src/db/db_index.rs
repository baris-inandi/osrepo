use super::Db;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use std::collections::HashMap;

impl Db {
    pub fn search(&self, keyword: &str) -> Result<HashMap<u8, String>, std::io::Error> {
        if keyword.len() < 1 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Search keyword is empty",
            ));
        }
        let keys = self.entries.keys();
        let first_letter = keyword.chars().next().unwrap();
        let mut keys_first_letter: Vec<&str> = Vec::new();
        for key in keys {
            if (&key).starts_with(first_letter) {
                keys_first_letter.push(&key);
            }
        }
        let matcher = SkimMatcherV2::default().ignore_case().use_cache(true);
        for key in keys_first_letter {
            let x = matcher.fuzzy_match("ğrch", "arch");
            println!("{:?}", x);
        }
        let out = HashMap::new();
        return Ok(out);
    }
}
