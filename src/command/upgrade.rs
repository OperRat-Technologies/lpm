use colored::Colorize;
use reqwest::Client;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::{fs, io};

pub async fn upgrade_lpm_installation() -> Result<(), Box<dyn std::error::Error>> {
    let owner = "OperRat-Technologies";
    let repo = "lpm";
    let current_version = env!("CARGO_PKG_VERSION");
    let current_version_tag_name = format!("v{}", current_version);

    println!("{}", "Checking for updates...".cyan());
    let latest_release = match get_latest_release(owner, repo).await {
        Ok(latest_release) => latest_release,
        Err(error) => {
            println!("{}: {}", "Failed to get latest release".red(), error);
            return Ok(());
        }
    };

    if current_version_tag_name == latest_release.tag_name {
        println!(
            "{} ({})",
            "You're already on the latest version!".cyan(),
            current_version_tag_name
        );
        return Ok(());
    }

    println!("New version available: {}", current_version_tag_name);

    let platform = get_platform();
    if platform == "unsupported" {
        eprintln!("Unsupported platform.");
        return Ok(());
    }

    let mut binary_url_search = None;
    let mut checksum_url_search = None;

    // Find binary and checksum assets
    for asset in &latest_release.assets {
        if asset.name.contains(platform) && !asset.name.ends_with(".sha256") {
            binary_url_search = Some(asset.browser_download_url.clone());
        } else if asset.name.contains(platform) && asset.name.ends_with(".sha256") {
            checksum_url_search = Some(asset.browser_download_url.clone());
        }
    }

    if binary_url_search.is_none() || checksum_url_search.is_none() {
        eprintln!("{}", "Failed to get latest release".red());
        return Ok(());
    }

    let binary_url = binary_url_search.unwrap();
    let checksum_url = checksum_url_search.unwrap();
    let binary_path = "new_version.bin";
    let checksum_path = "new_version.sha256";

    println!("{}", "Downloading new version...".cyan());

    match download_file(binary_url.as_str(), binary_path).await {
        Ok(..) => (),
        Err(error) => {
            eprintln!("{}: {}", "Failed to download new version".red(), error);
            return Ok(());
        }
    }

    match download_file(checksum_url.as_str(), checksum_path).await {
        Ok(..) => (),
        Err(error) => {
            eprintln!(
                "{}: {}",
                "Failed to download new version checksum".red(),
                error
            );
            return Ok(());
        }
    }

    println!("{}", "Validating checksum...".cyan());
    let expected_checksum = match fs::read_to_string(checksum_path) {
        Ok(checksum) => checksum.trim().to_string(),
        Err(error) => {
            eprintln!("{}: {}", "Failed to load checksum".red(), error);
            return Ok(());
        }
    };
    let actual_checksum = match calculate_sha256(binary_path) {
        Ok(sha256) => sha256,
        Err(error) => {
            eprintln!("{}: {}", "Failed to calculate sha256 hash".red(), error);
            return Ok(());
        }
    };
    if expected_checksum != actual_checksum {
        eprintln!("{}", "Checksum mismatch! Aborting update.".red());
        fs::remove_file(binary_path)?;
        fs::remove_file(checksum_path)?;
        return Ok(());
    }

    println!(
        "{}",
        "Checksum verified. Updating executable...".bright_green()
    );
    replace_current_executable(binary_path)?;

    println!("{}", "Restarting application...".yellow());
    restart_application()?;

    Ok(())
}

async fn get_latest_release(
    owner: &str,
    repo: &str,
) -> Result<Release, Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/releases/latest",
        owner, repo
    );

    let client = Client::new();
    let response = client.get(&url).header("User-Agent", "lpm").send().await?;
    let release: Release = response.json().await?;
    Ok(release)
}

fn get_platform() -> &'static str {
    if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else {
        "unsupported"
    }
}

async fn download_file(url: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let mut file = File::create(output_path)?;
    let content = response.bytes().await?;
    file.write_all(&content)?;
    Ok(())
}

fn calculate_sha256(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = fs::File::open(file_path)?;
    let mut hasher = Sha256::new();
    io::copy(&mut file, &mut hasher)?;
    let hash = hasher.finalize();
    Ok(format!("{:x}", hash))
}

fn replace_current_executable(new_executable_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let current_executable_path = std::env::current_exe()?;
    fs::rename(new_executable_path, current_executable_path)?;
    Ok(())
}

fn restart_application() -> Result<(), Box<dyn std::error::Error>> {
    let current_executable = std::env::current_exe()?;
    Command::new(current_executable).spawn()?;
    std::process::exit(0);
}

#[derive(Deserialize)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Deserialize)]
struct Asset {
    browser_download_url: String,
    name: String,
}
