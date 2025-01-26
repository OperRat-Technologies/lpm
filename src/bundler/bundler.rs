use crate::bundler::code_src_file::CodeSrcFile;
use colored::Colorize;
use regex::Regex;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Instant;
use walkdir::WalkDir;

pub struct LuaBundler {
    pub sources: HashMap<String, CodeSrcFile>,
}

impl LuaBundler {
    pub fn new() -> Self {
        Self {
            sources: HashMap::new(),
        }
    }

    pub fn bundle(&mut self, entry_point: &Path) -> Result<String, String> {
        let start_time = Instant::now();

        let mut entry_point_src = CodeSrcFile::new(entry_point.to_path_buf());
        match entry_point_src.parse() {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        entry_point_src.requires.iter().for_each(|dep| {
            self.recursive_dependency_check(dep);
        });

        self.sources
            .insert("entry_point".to_string(), entry_point_src);

        println!(
            "{} {} {}: [{}]",
            "Bundling".cyan(),
            self.sources.len() - 1,
            "modules".cyan(),
            self.sources
                .keys()
                .map(|k| k.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );

        let mut bundle = String::new();
        bundle.push_str("local modules = {}\n");

        self.sources.iter().for_each(|(module, src)| {
            bundle.push_str(format!("-- Begin: {} module\n", module).as_str());
            bundle.push_str(format!("modules['{}'] = (function()\n", module).as_str());

            let pattern = r#"require\(["'](.+)["']\)"#;
            let regex = Regex::new(pattern).unwrap();
            let module_transformed_code =
                regex.replace_all(&src.source, |caps: &regex::Captures| {
                    match self.sources.contains_key(&caps[1]) {
                        true => format!("modules[\"{}\"]()", &caps[1]),
                        false => format!("require('{}')", &caps[1]),
                    }
                });

            bundle.push_str(module_transformed_code.to_string().as_str());
            bundle.push_str("\nend)\n");
            bundle.push_str(format!("-- End: {} module\n", module).as_str());
        });

        bundle.push_str("\nmodules['entry_point']()\n");

        let duration = start_time.elapsed();
        println!(
            "{} {} {} {:?}",
            "Bundled".bright_green(),
            self.sources.len() - 1,
            "modules in".bright_green(),
            duration
        );

        Ok(bundle)
    }

    fn recursive_dependency_check(&mut self, dep_name: &String) {
        let module_path = match self.find_module(dep_name) {
            Some(module) => module,
            None => {
                println!("{} {}", "Unable to find module".yellow(), dep_name);
                return;
            }
        };
        println!("{} {}", "Found module".cyan(), dep_name);
        let mut module_src = CodeSrcFile::new(module_path.to_path_buf());
        match module_src.parse() {
            Ok(_) => (),
            Err(_) => return,
        }
        module_src.requires.iter().for_each(|dep| {
            self.recursive_dependency_check(dep);
        });
        self.sources.insert(dep_name.clone(), module_src);
    }

    fn find_module(&self, name: &String) -> Option<PathBuf> {
        let module_file_name = format!("{}.lua", name);
        let project_src = self.search_file_recursively(Path::new("."), &module_file_name);
        // todo: search dependencies
        project_src
    }

    fn search_file_recursively(&self, dir: &Path, target: &String) -> Option<PathBuf> {
        for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
            let file_name = entry.file_name().to_string_lossy();
            if file_name.to_string().as_str() == target.as_str() {
                return Some(entry.path().to_path_buf());
            }
        }
        None
    }
}
