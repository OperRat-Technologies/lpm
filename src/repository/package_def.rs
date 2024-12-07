use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageDef {
    pub(crate) info: RepoInfo
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoInfo {
    pub(crate) name: String,
    pub(crate) version: String
}