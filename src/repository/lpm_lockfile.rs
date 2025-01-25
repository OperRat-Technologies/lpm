use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct LPMLockfile {
    version: i32,
    packages: Vec<LPMLockfilePackage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LPMLockfilePackage {
    pub name: String,
    pub version: String,
    pub source: String,

    pub last_commit_hash: Option<String>,
}

impl LPMLockfile {
    pub fn new() -> Self {
        LPMLockfile {
            version: 1,
            packages: Vec::new(),
        }
    }

    pub fn load_from_file(path: &Path) -> Result<Self, String> {
        let lockfile_content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) => return Err(format!("Failed to read lockfile: {}", e.to_string())),
        };
        match toml::from_str::<LPMLockfile>(&lockfile_content) {
            Ok(lockfile) => Ok(lockfile),
            Err(e) => Err(format!("Failed to parse lockfile: {}", e.to_string())),
        }
    }

    pub fn write_to_file(&self, path: &Path) -> Result<(), String> {
        let lockfile_serialized = match toml::to_string(&self) {
            Ok(lockfile_serialized) => lockfile_serialized,
            Err(_) => return Err("Failed to serialize lockfile".to_string()),
        };
        match fs::write(path, lockfile_serialized) {
            Ok(_) => Ok(()),
            Err(_) => Err("Failed to write lockfile".to_string()),
        }
    }

    pub fn add_package(&mut self, package: LPMLockfilePackage) {
        self.packages.push(package);
    }
}
