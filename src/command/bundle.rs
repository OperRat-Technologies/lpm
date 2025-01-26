use crate::bundler::bundler::LuaBundler;
use crate::uploader;
use colored::Colorize;
use std::fs;
use std::path::Path;

pub async fn bundle_files(entry: &String, upload: &bool, out: &Option<String>) {
    let mut bundler = LuaBundler::new();
    let bundle_str = bundler.bundle(Path::new(&entry)).unwrap();

    if *upload {
        println!("{}", "Uploading bundle...".cyan());
        let url = uploader::dpaste::upload_to_dpaste(&bundle_str).await;
        println!("{}", url);
        return;
    }

    let output_file_name = if out.is_some() {
        out.clone().unwrap()
    } else {
        "bundle.lua".to_string()
    };

    fs::write(Path::new(".").join(output_file_name), &bundle_str).unwrap();
}
