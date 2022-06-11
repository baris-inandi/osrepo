use super::Db;

impl Db {
    pub async fn update(&self) -> Result<(), std::io::Error> {
        let url = &self.update_url;
        let backup_path = format!("{}.bak", &self.path);
        match url {
            Some(url) => {
                println!("Downloading database...");
                std::fs::copy(&self.path, backup_path)?;
                crate::utils::download::download(&self.client, url, &self.path)
                    .await
                    .unwrap();
                println!("Download finished.");
                println!("Testing database...");
                // TODO: perform testing here
                // let new_db = Db::new(&self.path);
                // new_db.load_entries();
            }
            None => println!("No update url found"),
        }
        return Ok(());
    }
}
