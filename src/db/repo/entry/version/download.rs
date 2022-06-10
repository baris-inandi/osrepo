use super::Version;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::cmp::min;
use std::fs::File;
use std::io::Write;

impl Version {
    pub async fn download(&self, client: &Client) -> Result<(), String> {
        let path = "download";
        let url = "http://ftp.agdsn.de/pub/mirrors/archlinux/iso/2022.06.01/archlinux-2022.06.01-x86_64.iso";

        let res = client
            .get(url)
            .send()
            .await
            .or(Err(format!("Failed to GET from '{}'", &url)))?;
        let total_size = res
            .content_length()
            .ok_or(format!("Failed to get content length from '{}'", &url))?;

        let pb = ProgressBar::new(total_size);
        pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "\n{elapsed_precise:.green} elapsed, {eta:.yellow} left ({bytes}/{total_bytes})\n{percent}%  █{bar:25}  {bytes_per_sec}\n",
            )
            .progress_chars("█▓░"),
    );
        pb.set_message("");

        // download chunks
        let mut file = File::create(path).or(Err(format!("Failed to create file '{}'", path)))?;
        let mut downloaded: u64 = 0;
        let mut stream = res.bytes_stream();

        while let Some(item) = stream.next().await {
            let chunk = item.or(Err(format!("Error while downloading file")))?;
            file.write_all(&chunk)
                .or(Err(format!("Error while writing to file")))?;
            let new = min(downloaded + (chunk.len() as u64), total_size);
            downloaded = new;
            pb.set_position(new);
        }
        pb.finish_with_message("[Done]");
        println!("Download complete.\n");
        return Ok(());
    }
}
