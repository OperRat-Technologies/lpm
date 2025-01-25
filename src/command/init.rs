use crate::repository::lpm_repository::{LPMRepository, LPMRepositoryInfo};
use colored::Colorize;
use std::io;
use std::io::Write;
use std::path::Path;

pub fn init_repository(path: &Path) {
    println!("{}", "Initializing repository".bright_green());

    let mut package_name: String = String::new();
    print!("{}", "Package name: ".bright_yellow());
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut package_name).unwrap();
    package_name = package_name.trim().to_string();

    let mut package_version = String::new();
    print!("{}", "Package version: ".bright_yellow());
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut package_version).unwrap();
    package_version = package_version.trim().to_string();

    let repo_info = LPMRepositoryInfo {
        name: package_name,
        version: package_version,
    };

    let pkg_def = LPMRepository::new(repo_info, path.to_path_buf());

    match pkg_def.write_to_file() {
        Ok(p) => p,
        Err(e) => {
            println!("{}: {}", "Failed to create repository".red(), e);
            return;
        }
    };

    println!(
        "Initialized repository for {} version {}",
        pkg_def.info.name.yellow(),
        pkg_def.info.version.yellow()
    );
}
