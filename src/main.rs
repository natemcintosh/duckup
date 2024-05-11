use std::{
    fs::{self, File},
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

use clap::Parser;
use reqwest::{blocking::Client, header::USER_AGENT};
use tempfile::tempdir;
use zip::read::ZipArchive;

use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Release {
    pub url: String,
    #[serde(rename = "assets_url")]
    pub assets_url: String,
    #[serde(rename = "upload_url")]
    pub upload_url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    pub id: i64,
    pub author: Author,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "tag_name")]
    pub tag_name: String,
    #[serde(rename = "target_commitish")]
    pub target_commitish: String,
    pub name: String,
    pub draft: bool,
    pub prerelease: bool,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "published_at")]
    pub published_at: String,
    pub assets: Vec<Asset>,
    #[serde(rename = "tarball_url")]
    pub tarball_url: String,
    #[serde(rename = "zipball_url")]
    pub zipball_url: String,
    pub body: String,
    pub reactions: Reactions,
    #[serde(rename = "mentions_count")]
    pub mentions_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub login: String,
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    #[serde(rename = "gravatar_id")]
    pub gravatar_id: String,
    pub url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "followers_url")]
    pub followers_url: String,
    #[serde(rename = "following_url")]
    pub following_url: String,
    #[serde(rename = "gists_url")]
    pub gists_url: String,
    #[serde(rename = "starred_url")]
    pub starred_url: String,
    #[serde(rename = "subscriptions_url")]
    pub subscriptions_url: String,
    #[serde(rename = "organizations_url")]
    pub organizations_url: String,
    #[serde(rename = "repos_url")]
    pub repos_url: String,
    #[serde(rename = "events_url")]
    pub events_url: String,
    #[serde(rename = "received_events_url")]
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "site_admin")]
    pub site_admin: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub url: String,
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: String,
    pub name: String,
    pub label: String,
    pub uploader: Uploader,
    #[serde(rename = "content_type")]
    pub content_type: String,
    pub state: String,
    pub size: i64,
    #[serde(rename = "download_count")]
    pub download_count: i64,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "browser_download_url")]
    pub browser_download_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Uploader {
    pub login: String,
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    #[serde(rename = "gravatar_id")]
    pub gravatar_id: String,
    pub url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "followers_url")]
    pub followers_url: String,
    #[serde(rename = "following_url")]
    pub following_url: String,
    #[serde(rename = "gists_url")]
    pub gists_url: String,
    #[serde(rename = "starred_url")]
    pub starred_url: String,
    #[serde(rename = "subscriptions_url")]
    pub subscriptions_url: String,
    #[serde(rename = "organizations_url")]
    pub organizations_url: String,
    #[serde(rename = "repos_url")]
    pub repos_url: String,
    #[serde(rename = "events_url")]
    pub events_url: String,
    #[serde(rename = "received_events_url")]
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "site_admin")]
    pub site_admin: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reactions {
    pub url: String,
    #[serde(rename = "total_count")]
    pub total_count: i64,
    #[serde(rename = "+1")]
    pub n1: i64,
    #[serde(rename = "-1")]
    pub n12: i64,
    pub laugh: i64,
    pub hooray: i64,
    pub confused: i64,
    pub heart: i64,
    pub rocket: i64,
    pub eyes: i64,
}

fn get_latest_release_zip_urls() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // Construct the URL for GitHub API to fetch the latest release information
    let url = "https://api.github.com/repos/duckdb/duckdb/releases/latest";

    // Send a GET request to fetch the latest release information
    let client = Client::new();
    let text = client
        .get(url)
        .header(USER_AGENT, "duckup")
        .send()?
        .text()?;
    let release: Release = serde_json::from_str(&text)?;

    // Extract the URL of the zip files from the release information
    Ok(release
        .assets
        .iter()
        .map(|a| a.browser_download_url.clone())
        .collect())
}

fn download_zip(url: &str, output_dir: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Create a reqwest client
    let client = Client::new();

    // Send a GET request to the provided URL
    let mut response = client.get(url).send()?;

    // Check if the request was successful
    if !response.status().is_success() {
        return Err("Failed to download file".into());
    }

    // Open a file to write the downloaded content
    let zip_file_path = output_dir.join("downloaded.zip");
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

fn get_matching_url<'a>(
    zip_urls: &'a [String],
    info: &'a os_info::Info,
) -> Result<&'a str, Box<dyn std::error::Error>> {
    Ok(zip_urls
        .iter()
        // Filter to urls with this OS
        .filter(|url| url.contains("linux"))
        // Only want the CLI binaries
        .filter(|url| url.contains("cli"))
        // Filter to the correct architecture
        // Can't seem to find a list of what all the possible architecture might be,
        // so I'm mostly guessing here.
        .filter(|url| match info.architecture() {
            Some("arm64") | Some("aarch") => url.contains("aarch"),
            Some("amd") | Some("x86_64") => url.contains("amd"),
            Some(&_) | None => false,
        })
        .next()
        .ok_or("Could not find any matching URLs")?)
}

#[derive(clap::Parser)]
#[command(version, about, long_about = None, arg_required_else_help(true))]
struct CLI {
    /// Run the update, downloading the latest binary, and installing it
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Runs the update
    Update,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = CLI::parse();

    match cli.command {
        // All is good, just continue on with what is below
        Some(Commands::Update) => {}
        // Error out
        None => panic!("Did not receive an expected command"),
    }

    let info = os_info::get();

    // Check that we are only running on Linux
    if &info.os_type().to_string() == "Windows" {
        panic!("Running on Windows. Will only work on Linux.")
    } else if &info.os_type().to_string() == "Mac OS" {
        panic!("Running on Mac. Will only work on Linux.")
    }

    let zip_urls: Vec<String> = get_latest_release_zip_urls()?.into_iter().collect();
    // Get the correct url for this architecture
    let url = get_matching_url(&zip_urls, &info).expect("Could not find a URL for this computer");
    println!("Going to download {}", url);

    // Create a temp directory for unzipping
    let zip_dir = tempdir().expect("Could not make tempdir to put zip file in");
    std::fs::create_dir_all(zip_dir.path()).expect("Could not create folder within tempdir");
    // Download the file
    let zip_file_path =
        download_zip(url, &zip_dir.path()).expect("Failed to download the zip file");

    println!("Downloaded successfully!");

    let final_path = home::home_dir()
        .expect("Could not find home directory")
        .join(".local")
        .join("bin/");
    std::fs::create_dir_all(final_path.clone())?;
    unzip_file(&zip_file_path, &final_path.clone())?;
    println!("Unzipped successfully into {:?}", final_path);

    // Make file executable
    let mut perms = fs::metadata(final_path.clone())?.permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(final_path.clone(), perms)
        .expect("Could not set the duckdb executable as executable");
    std::process::Command::new("chmod")
        .args([
            "+x",
            final_path.to_str().expect("Failed to convert to &str"),
        ])
        .status()
        .expect("Unable to set permissions");

    Ok(())
}
