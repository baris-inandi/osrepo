use colored::Colorize;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use reqwest::StatusCode;
use std::cmp::min;
use std::io::Write;

pub async fn download(client: &Client, url: &str, target_path: &str) -> Result<(), String> {
    let res = client
        .get(url)
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", &url)))?;
    match res.status() {
        StatusCode::OK => {}
        code => {
            println!(
                "Download failed: Server responded with {}",
                code.to_string().red()
            );
            return Err(format!("Request failed with '{}'", code));
        }
    };
    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", &url))?;

    let pb = ProgressBar::new(total_size);
    pb.set_style(
            ProgressStyle::default_bar()
            .template(
                "\n{elapsed_precise:.green} elapsed, {eta:.yellow} left ({bytes}/{total_bytes})\n{percent}%  █{bar:33}  {bytes_per_sec}\n\n",
            )
            .progress_chars("█▓░"),
        );
    pb.set_message("");

    // download chunks
    let mut file = crate::utils::fs::expand_create_file(target_path).or(Err(format!(
        "Cannot create target file at '{}'",
        target_path
    )))?;
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
    return Ok(());
}
