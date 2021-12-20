use super::debug;
use git2::Repository;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use users::get_current_username;

fn create_file_contents(name: String) -> String {
    let username = get_current_username().unwrap();

    format!(
        "# package.yml
package:
    name: {}
    description: 'An example package for math in yf'
    authors:
        - {:?}

    license: 'MIT'

# Dependencies for this package
depends:
",
        name, username
    )
}

fn git_init_package() {
    match Repository::init(".") {
        Ok(_) => {
            debug!("Initialized git");
            Command::new("git")
                .args(["checkout", "-b", "main"])
                .output()
                .expect("failed to execute process");
        }
        Err(e) => panic!("failed to init: {}", e),
    }
}

fn create_package_contents(name: String) -> std::io::Result<()> {
    // Create package.yml and src
    debug!("Creating directory package.yml");
    let mut file = File::create("package.yml")?;
    let contents = create_file_contents(name);
    write!(file, "{}", contents)?;

    debug!("Creating src/ folder");
    fs::create_dir_all("./src")?;

    git_init_package();

    Ok(())
}

pub fn init(name: Option<String>) -> std::io::Result<()> {
    let project_name;
    if name.is_none() {
        let path = env::current_dir()?;
        let path_name = path.components().next_back().unwrap();
        debug!(
            "Name not specified, working in current directory {:?}",
            path_name.as_os_str()
        );
        project_name = format!("{:?}", path_name.as_os_str());
    } else {
        debug!("Creating directory called {}", name.clone().unwrap());
        fs::create_dir_all(format!("./{}", name.clone().unwrap()))?;
        env::set_current_dir(name.clone().unwrap())?;
        project_name = name.unwrap();
    }

    match create_package_contents(project_name) {
        Ok(_) => println!("Package created successfully!"),
        Err(e) => eprintln!("{}", e),
    };

    Ok(())
}
