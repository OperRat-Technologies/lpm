use crate::repository::package_def::{PackageDef, RepoInfo};
use std::fs;
use std::path::Path;

pub fn check_if_folder_is_a_repo(path: &Path) -> bool {
    let package_path = path.join("package.toml");
    package_path.exists() && package_path.is_file()
}

pub fn parse_package_file(path: &Path) -> Result<PackageDef, String> {
    let package_content = match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(_) => return Err("Failed to read package.toml".to_string()),
    };
    match toml::from_str::<PackageDef>(&package_content) {
        Ok(package_def) => Ok(package_def),
        Err(_) => Err("Invalid package file".to_string()),
    }
}

pub fn create_repo(path: &Path, info: RepoInfo) -> Result<PackageDef, String> {
    let package_body = PackageDef { info };

    let package_serialized = match toml::to_string(&package_body) {
        Ok(package_serialized) => package_serialized,
        Err(_) => return Err("Failed to serialize package.toml".to_string()),
    };

    let package_file = path.join("package.toml");

    match fs::write(&package_file, package_serialized) {
        Ok(_) => Ok(package_body),
        Err(_) => Err("Failed to write package.toml".to_string()),
    }
}
