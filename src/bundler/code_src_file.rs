use regex::Regex;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct CodeSrcFile {
    path: PathBuf,

    pub source: String,
    pub requires: Vec<String>,
}

impl CodeSrcFile {
    pub fn new(path: PathBuf) -> CodeSrcFile {
        CodeSrcFile {
            path,
            source: String::new(),
            requires: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Result<(), String> {
        match self.parse_file() {
            Ok(_) => (),
            Err(e) => return Err(format!("{}: {:?}", "Failed to parse file", e)),
        }
        match self.query_dependencies() {
            Ok(_) => (),
            Err(e) => return Err(format!("{}: {:?}", "Failed to query dependencies", e)),
        }
        Ok(())
    }

    fn parse_file(&mut self) -> Result<(), String> {
        self.source = match fs::read_to_string(&self.path) {
            Ok(content) => content,
            Err(e) => {
                return Err(e.to_string());
            }
        };
        Ok(())
    }

    fn query_dependencies(&mut self) -> Result<(), String> {
        let pattern = r#"require\(["'](.+)["']\)"#;
        let regex = match Regex::new(pattern) {
            Ok(regex) => regex,
            Err(_) => return Err(format!("Invalid regular expression: {}", pattern)),
        };
        let requires: Vec<Vec<String>> = regex
            .captures_iter(&self.source)
            .map(|caps| {
                // For each match, collect all capture groups into a Vec<String>
                caps.iter()
                    .skip(1)
                    .filter_map(|m| m.map(|mat| mat.as_str().to_string()))
                    .collect()
            })
            .collect();

        requires.iter().for_each(|group| {
            group.iter().for_each(|mat| {
                self.requires.push(String::from(mat));
            });
        });

        Ok(())
    }
}
