use super::Version;
use reqwest::Client;

impl Version {
    pub async fn download(&self, client: &Client) -> Result<(), String> {
        let path = "~/code/osrepo/download";
        let url = "http://ftp.agdsn.de/pub/mirrors/archlinux/iso/2022.06.01/archlinux-2022.06.01-x86_64.iso";
        crate::utils::download::download(client, url, path).await?;
        println!("Download complete.\n");
        return Ok(());
    }
}
