use crate::luarocks::luarocks;
use crate::repository::repository;
use colored::Colorize;
use std::path::Path;

fn validate_package_name(name: &str) -> Result<(&str, &str), &str> {
    let package_name_parts = name.split('/').collect::<Vec<&str>>();

    if package_name_parts.len() < 2 {
        return Err("Non-namespaced packages are not yet supported, please add packages as NAMESPACE/PACKAGE");
    }

    if package_name_parts.len() != 2 {
        return Err("Unknown package format, please use as NAMESPACE/PACKAGE");
    }

    Ok((package_name_parts[0].trim(), package_name_parts[1].trim()))
}

pub async fn add_package(name: &str, version: &Option<String>) {
    if !repository::check_if_folder_is_a_repo(Path::new(".")) {
        println!("{}", "Current folder is not a repository".yellow());
        return;
    }

    let (pkg_namespace, pkg_name) = match validate_package_name(name) {
        Ok(x) => x,
        Err(e) => {
            println!("{}: {}", "Failed to validate package name".red(), e);
            return;
        }
    };

    println!("{}", "Loading remote repository...".dimmed());

    let repo =
        match luarocks::load_namespace_repository("https://luarocks.org", pkg_namespace).await {
            Ok(x) => x,
            Err(e) => {
                println!("{}: {}", "Failed to load remote repository".red(), e);
                return;
            }
        };

    println!("{}", "Searching for package...".dimmed());

    let repo_pkg = match repo.get_package_by_name(pkg_name.to_lowercase().as_str()) {
        Some(p) => p,
        None => {
            println!(
                "{} {} {} {}",
                "Namespace".red(),
                pkg_namespace,
                "doesn't have the package".red(),
                &pkg_name
            );
            return;
        }
    };

    let pkg_version_get = match &version {
        Some(v) => repo_pkg.get_specific_package_version(v),
        None => repo_pkg.get_latest_package_version(),
    };

    let pkg_version = match pkg_version_get {
        Some(v) => v,
        None => {
            println!(
                "{} {} {} {}, {}: {:?}",
                "Failed to get version".red(),
                version.clone().unwrap(),
                "from package".red(),
                &pkg_name,
                "Available versions",
                repo_pkg
                    .versions
                    .iter()
                    .map(|v| v.version.clone())
                    .collect::<Vec<String>>()
                    .join(", ")
            );
            std::process::exit(0);
        }
    };

    println!("{:?}", pkg_version);
}
