use super::{debug};
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use users::get_current_username;

fn create_file_contents(name: String) -> String {
    let username = get_current_username();

    format!(
        "
# package.yml \
package: \
    name: '{}' \
    description: 'An example package for math in yf' \
    authors: \
        - '{:?}' \
    license: 'MIT' \

# Dependencies for this package \
depends:",
        name, username
    )
}

fn create_package_contents(name: String) -> std::io::Result<()> {
    // Create package.yml and src
    debug!("Creating directory package.yml");
    let mut file = File::create("package.yml")?;
    let contents = create_file_contents(name);
    write!(file, "{}", contents)?;

    debug!("Creating src/ folder");
    fs::create_dir_all("./src")?;

    Ok(())
}

pub fn init(name: Option<String>) -> std::io::Result<()> {
    let project_name;
    if name.is_none() {
        let path = env::current_dir()?;
        debug!(
            "Name not specified, working in current directory {:?}",
            path
        );
        project_name = path.display().to_string();
    } else {
        debug!("Creating directory called {}", name.clone().unwrap());
        fs::create_dir_all(format!("/{:?}", name))?;
        project_name = name.unwrap();
    }

    match create_package_contents(project_name) {
        Ok(_) => println!("Package created successfully!"),
        Err(e) => eprintln!("{}", e),
    };

    Ok(())
}
