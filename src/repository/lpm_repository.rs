use crate::luarocks::formats::rockspec::RockSpec;
use crate::repository::downloaders::git::LPMGitDownloader;
use crate::repository::downloaders::lpm_downloader::LPMDownloader;
use crate::repository::lpm_lockfile::{LPMLockfile, LPMLockfilePackage};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct LPMRepository {
    pub info: LPMRepositoryInfo,
    pub dependencies: HashMap<String, String>,

    #[serde(skip_serializing, skip_deserializing)]
    path: PathBuf,

    #[serde(skip_serializing, skip_deserializing)]
    lockfile: LPMLockfile,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LPMRepositoryInfo {
    pub name: String,
    pub version: String,
}

impl LPMRepository {
    pub fn new(info: LPMRepositoryInfo, path: PathBuf) -> Self {
        LPMRepository {
            info,
            dependencies: HashMap::new(),

            path,
            lockfile: LPMLockfile::new(),
        }
    }

    pub fn load_from_path(path: &Path) -> Result<Self, String> {
        let package_path = path.join("package.toml");

        let package_content = match fs::read_to_string(&package_path) {
            Ok(content) => content,
            Err(e) => return Err(format!("Failed to read package.toml: {}", e.to_string())),
        };

        let mut loaded_repository = match toml::from_str::<LPMRepository>(&package_content) {
            Ok(package_def) => package_def,
            Err(_) => return Err("Invalid package file".to_string()),
        };

        loaded_repository.path = path.to_path_buf();

        let lockfile_path = path.join("package.lock.toml");
        match LPMLockfile::load_from_file(lockfile_path.as_path()) {
            Ok(lockfile) => loaded_repository.lockfile = lockfile,
            Err(_) => loaded_repository.lockfile = LPMLockfile::new(),
        }

        Ok(loaded_repository)
    }

    pub fn write_to_file(&self) -> Result<(), String> {
        let package_serialized = match toml::to_string(&self) {
            Ok(package_serialized) => package_serialized,
            Err(_) => return Err("Failed to serialize package.toml".to_string()),
        };

        let package_path = self.get_package_file_path();

        match fs::write(&package_path, package_serialized) {
            Ok(_) => (),
            Err(_) => return Err("Failed to write package.toml".to_string()),
        }

        match self.lockfile.write_to_file(&self.get_lockfile_file_path()) {
            Ok(_) => (),
            Err(e) => {
                return Err(format!(
                    "{}: {}",
                    "Failed to write lockfile".red(),
                    e.to_string()
                ))
            }
        }

        Ok(())
    }

    pub fn add_package(&mut self, rockspec: &RockSpec) -> Result<(), String> {
        let installed_version = self.get_repository_package_version(&rockspec.package);
        if installed_version.is_none() {
            match self.download_package(rockspec) {
                Ok(_) => (),
                Err(e) => return Err(format!("{}: {}", "Failed to download package".red(), e)),
            }

            self.dependencies
                .insert(rockspec.package.clone(), rockspec.version.clone());

            match self.write_to_file() {
                Ok(_) => (),
                Err(e) => return Err(format!("{}: {}", "Failed to write package".red(), e)),
            }
        }
        Ok(())
    }

    fn download_package(&mut self, rockspec: &RockSpec) -> Result<(), String> {
        let is_git = rockspec.source.url.starts_with("git+");
        if !is_git {
            return Err("Only git packages are supported for now".to_string());
        }

        let url = rockspec.source.url.replace("git+", "");

        let lua_rocks_folder = match self.get_lua_rocks_folder() {
            Ok(folder) => folder,
            Err(_) => return Err("Failed to get lua rocks folder".to_string()),
        };

        let pkg_folder = lua_rocks_folder.join(&rockspec.package);

        let last_commit_hash = match LPMGitDownloader::download(url.as_str(), &pkg_folder) {
            Ok(hash) => hash,
            Err(_) => return Err("Failed to download lua rocks folder".to_string()),
        };

        let lock_data = LPMLockfilePackage {
            name: rockspec.package.clone(),
            source: rockspec.source.url.clone(),
            version: rockspec.version.clone(),
            last_commit_hash: Some(last_commit_hash),
        };

        self.lockfile.add_package(lock_data);

        Ok(())
    }

    fn get_repository_package_version(&self, name: &String) -> Option<String> {
        self.dependencies.get(name.as_str()).cloned()
    }

    fn get_package_file_path(&self) -> PathBuf {
        self.path.join("package.toml")
    }

    fn get_lockfile_file_path(&self) -> PathBuf {
        self.path.join("package.lock.toml")
    }

    fn get_lua_rocks_folder(&self) -> Result<PathBuf, String> {
        let path = self.path.join("lua_rocks");
        if !path.exists() {
            return match fs::create_dir_all(&path) {
                Ok(_) => Ok(path),
                Err(_) => Err("Failed to create lua rocks folder".to_string()),
            };
        }
        Ok(path)
    }

    pub fn is_folder_repository(path: &Path) -> bool {
        let package_path = path.join("package.toml");
        package_path.exists() && package_path.is_file()
    }
}
