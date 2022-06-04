pub mod repo;
use repo::Repo;
use serde_yaml::Value;

pub struct Db {
    pub repos: Vec<Repo>,
}

impl Db {
    /*
        Example db_file:

        include:
            - ~/.config/osr/repos/apple.yml
            - ~/.config/osr/repos/extra.yml
            - ~/.config/osr/repos/gnulinux.yml
            - ~/.config/osr/repos/microsoft.yml
    */
    pub fn new(filepath: &str) -> Result<Db, serde_yaml::Error> {
        let file = crate::utils::expand_open_file(filepath).expect(&format!(
            "Cannot open db file \"{}\", does it exist?",
            &filepath
        ));
        let value: Value = serde_yaml::from_reader(file)?;
        let repos_value = value
            .get("include")
            .unwrap()
            .as_sequence()
            .unwrap()
            .to_vec();
        let mut repos: Vec<Repo> = Vec::new();
        for path in repos_value {
            repos.push(Repo::new(path.as_str().unwrap()));
        }
        return Ok(Db { repos });
    }
}
