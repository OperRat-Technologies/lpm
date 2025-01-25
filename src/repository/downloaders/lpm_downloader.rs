use std::path::Path;

pub trait LPMDownloader {
    fn download(url: &str, path: &Path) -> Result<String, String>;
}
