use std::fs;
use std::path::Path;
use colored::Colorize;
use crate::repository::package_def::{PackageDef, RepoInfo};

pub fn check_if_folder_is_a_repo(path: &Path) -> bool {
    let package_path = path.join("package.toml");
    package_path.exists() && package_path.is_file()
}

pub fn validate_package_file(path: &Path) -> PackageDef {
    let package_content = fs::read_to_string(&path).unwrap();
    match toml::from_str::<PackageDef>(&package_content) {
        Ok(package_def) => { package_def }
        Err(_err) => { panic!("Failed to parse package.toml"); }
    }
}

pub fn create_repo(path: &Path, info: RepoInfo) -> PackageDef {
    let package_body = PackageDef {
        info,
    };
    let package_serialized = toml::to_string(&package_body).unwrap();
    let package_file = path.join("package.toml");
    let write_op = fs::write(&package_file, package_serialized);
    match write_op {
        Ok(_) => {}
        Err(e) => {
            println!("{}: {}", "Failed to create package file".red(), e);
            std::process::exit(1);
        }
    }
    package_body
}