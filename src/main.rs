use std::{
    fs::File,
    path::{Path, PathBuf},
};

use reqwest::blocking::Client;
use tempfile::tempdir;
use zip::read::ZipArchive;

#[derive(Deserialize)]
struct Release {
    assets: Vec<Asset>,
}

#[derive(Deserialize)]
struct Asset {
    browser_download_url: String,
}

fn get_latest_release_zip_url(
    owner: &str,
    repo: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Construct the URL for GitHub API to fetch the latest release information
    let url = "https://api.github.com/repos/duckdb/duckdb/releases/latest";

    // Send a GET request to fetch the latest release information
    let response = reqwest::blocking::get(url)?;
    let release: Release = response.json()?;

    // Extract the URL of the zip file from the release information
    if let Some(asset) = release
        .assets
        .into_iter()
        .find(|asset| asset.browser_download_url.ends_with(".zip"))
    {
        Ok(asset.browser_download_url)
    } else {
        Err("No zip file found in the latest release".into())
    }
}

fn download_zip(url: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Create a reqwest client
    let client = Client::new();

    // Send a GET request to the provided URL
    let mut response = client.get(url).send()?;

    // Check if the request was successful
    if !response.status().is_success() {
        return Err("Failed to download file".into());
    }

    // Open a file to write the downloaded content
    let temp_dir = tempdir()?;
    let zip_file_path = temp_dir.path().join("downloaded.zip");
    let mut file = File::create(&zip_file_path)?;

    // Copy the content of the response to the file
    let _ = response.copy_to(&mut file);

    Ok(zip_file_path)
}

fn unzip_file(zip_file: &Path, output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(zip_file)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = Path::new(output_dir).join(file.name());

        if let Some(parent_dir) = outpath.parent() {
            if !parent_dir.exists() {
                std::fs::create_dir_all(parent_dir)?;
            }
        }

        if file.is_dir() {
            std::fs::create_dir_all(&outpath)?;
        } else {
            let mut outfile = File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url =
        "https://github.com/duckdb/duckdb/releases/download/v0.10.2/duckdb_cli-linux-amd64.zip";

    let zip_file_path = download_zip(url)?;

    println!("Downloaded successfully!");

    // Create a temp directory for unzipping
    let temp_dir = tempdir()?;
    let output_dir = temp_dir.path().join("output");
    unzip_file(&zip_file_path, &output_dir)?;
    println!("Unzipped successfully!");

    Ok(())
}
