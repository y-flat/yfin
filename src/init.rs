use super::{debug, error, info, warn};
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

pub fn create_contents(name: String) {
    // Create package.yml and src
    let mut file = File::create("package.yml")?;
    debug!("Creating directory package.yml");
    fs::create_dir_all("/src")?;
    debug!("Creating src/ folder");
}

pub fn init(name: Option<String>) -> std::io::Result<()> {
    if name.is_none() {
        debug!("Name not specified, working in current directory");
        let path = env::current_dir()?;
        println!("The current directory is {}", path.display());
        let project_name: String = path.display();
    } else {
        // Create directory with name
        fs::create_dir_all(format!("/{}", name))?;
        debug!("Creating directory called {}", name);

        let project_name: String = name;
        create_contents(project_name);
    }
    Ok(())
}
