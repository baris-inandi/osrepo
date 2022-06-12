use super::Db;
use colored::Colorize;

impl Db {
    fn rollback(&self) {
        println!("{}", "Rolling back to backup database".red());
        std::fs::copy(&self.backup_path, &self.path).unwrap();
        println!("{}", "Aborted update".red());
    }
    pub async fn update(&self) -> Result<(), std::io::Error> {
        let url = &self.update_url;
        match url {
            Some(url) => {
                println!("Downloading database...");
                std::fs::copy(&self.path, &self.backup_path)?;
                crate::utils::download::download(&self.client, url, &self.path)
                    .await
                    .or_else(|_| {
                        println!("{}", "Cannot download new database".red());
                        self.rollback();
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "Failed to download update from update URL",
                        ));
                    })?;
                println!("Download finished");
                println!("Testing updated database...");
                Db::new(&self.path).or_else(|_| {
                    println!(
                        "{}",
                        "Test failed: New database cannot be instantiated".red()
                    );
                    self.rollback();
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "New database failed test: Cannot load Db",
                    ));
                })?;
                println!("{}", "Updated database successfully".green());
            }
            None => {
                println!("{}", "No update URL provided in database, aborting".red());
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "No update URL provided in database",
                ))?;
            }
        }
        return Ok(());
    }
}
