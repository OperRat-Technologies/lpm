use crate::repository::downloaders::lpm_downloader::LPMDownloader;
use git2::Repository;
use std::path::Path;

pub enum LPMGitDownloader {}

impl LPMDownloader for LPMGitDownloader {
    fn download(url: &str, path: &Path) -> Result<String, String> {
        let repo = match Repository::clone(url, path) {
            Ok(repo) => repo,
            Err(e) => return Err(e.to_string()),
        };

        let head = match repo.head() {
            Ok(head) => head,
            Err(e) => return Err(e.to_string()),
        };

        let head_commit = match head.peel_to_commit() {
            Ok(commit) => commit,
            Err(e) => return Err(e.to_string()),
        };

        let head_oid = head_commit.id();
        Ok(head_oid.to_string())
    }
}
