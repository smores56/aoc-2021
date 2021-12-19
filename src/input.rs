use std::fs::{create_dir_all, read_to_string, try_exists, write};

use anyhow::Context;

pub async fn get_input(day: usize) -> anyhow::Result<String> {
    let path = format!("input/{}.txt", day);
    if matches!(try_exists(&path), Ok(true)) {
        return read_to_string(&path).context("Failed to read input from file");
    }

    let input = retrieve_input(day).await?;

    create_dir_all("input").context("Failed to create input folder")?;
    write(path, &input).context("Failed to save input locally")?;

    Ok(input)
}

async fn retrieve_input(day: usize) -> anyhow::Result<String> {
    let session = std::env::var("SESSION").context("SESSION environment variable missing")?;
    let url = format!("https://adventofcode.com/2021/day/{}/input", day);

    let client = reqwest::Client::new();
    let request = client
        .get(url)
        .header("Cookie", format!("session={}", session));

    request
        .send()
        .await
        .context("Error getting input")?
        .text()
        .await
        .context("Failed to parse text")
}
