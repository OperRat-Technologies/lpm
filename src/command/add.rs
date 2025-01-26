use crate::luarocks::luarocks;
use crate::luarocks::luarocks::load_package_rockspec;
use crate::repository::lpm_repository::LPMRepository;
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
    let repo_url = "https://luarocks.org";
    if !LPMRepository::is_folder_repository(Path::new(".")) {
        println!("{}", "Current folder is not a repository".yellow());
        return;
    }

    let mut lpm_repo = match LPMRepository::load_from_cur_dir() {
        Ok(lpm_repo) => lpm_repo,
        Err(e) => {
            println!("{}: {}", "Failed to load local repository".red(), e);
            return;
        }
    };

    let (pkg_namespace, pkg_name) = match validate_package_name(name) {
        Ok(x) => x,
        Err(e) => {
            println!("{}: {}", "Failed to validate package name".red(), e);
            return;
        }
    };

    println!("{}", "Loading remote repository...".dimmed());

    let repo = match luarocks::load_namespace_repository(repo_url, pkg_namespace).await {
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
            return;
        }
    };

    let pkg_rockspec = match load_package_rockspec(
        repo_url.to_lowercase().as_str(),
        pkg_namespace.to_lowercase().as_str(),
        pkg_name.to_lowercase().as_str(),
        pkg_version.version.as_str(),
    )
    .await
    {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load package Rock spec: {}", e);
            return;
        }
    };

    match lpm_repo.add_package(&pkg_rockspec) {
        Ok(_) => {
            println!(
                "{} {} {}",
                "Successfully added package".green(),
                pkg_namespace,
                pkg_version.version.as_str()
            );
        }
        Err(e) => {
            println!("{}: {}", "Failed to add package".red(), e);
            return;
        }
    }
}
