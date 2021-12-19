use super::{debug, error, info, warn};
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use users::get_current_username;

fn create_file(name: String) -> String {
    let username = get_current_username();

    format!(
        "
# package.yml \
package: \
    name: '{}' \
    description: 'An example package for math in yf' \
    authors: \
        - '{}' \
    license: 'MIT' \

# Dependencies for this package \
depends:",
        name, username
    );
}

fn create_package_contents(name: String) {
    // Create package.yml and src
    debug!("Creating directory package.yml");
    let mut file = File::create("package.yml")?;
    file.write_all(create_file_contents(name));

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

    create_package_contents(project_name);

    Ok(())
}
