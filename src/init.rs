use super::{debug, error, info, warn};
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

pub fn create_contents(name: String) {
    // Create package.yml and src
    debug!("Creating directory package.yml");
    let mut file = File::create("package.yml")?;

    debug!("Creating src/ folder");
    fs::create_dir_all("/src")?;
}

pub fn init(name: Option<String>) -> std::io::Result<()> {
    if name.is_none() {
        let path = env::current_dir()?;
        debug!("Name not specified, working in current directory {}", path);

        let project_name: String = path.display();
    } else {
        debug!("Creating directory called {}", name);
        fs::create_dir_all(format!("/{}", name))?;

        let project_name: String = name;
    }

    create_contents(project_name);

    Ok(())
}
