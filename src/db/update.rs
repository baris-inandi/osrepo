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
                
            }
            None => println!("No update url found"),
        }
        return Ok(());
    }
}
