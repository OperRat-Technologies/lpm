use std::fs;
use std::path::Path;

pub fn clear_local_repository() {
    match fs::remove_dir_all(Path::new(".").join("/lua_rocks")) {
        Ok(_) => println!("Successfully removed local repository"),
        Err(why) => println!("Failed to remove local repository: {:?}", why.kind()),
    }
}
