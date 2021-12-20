use super::debug;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use users::get_current_username;

fn create_main_file_contents() -> String {
    String::from(
        "use std.io;

main() {
    io.println(\"Hello, World!\");
}",
    )
}

fn create_package_file_contents(name: String) -> String {
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

fn create_package_file(name: String) -> std::io::Result<()> {
    debug!("Creating package.yml");
    let mut package_file = File::create("package.yml")?;
    let package_contents = create_package_file_contents(name);
    write!(package_file, "{}", package_contents)?;
    Ok(())
}

fn create_main_file() -> std::io::Result<()> {
    debug!("Creating main file");
    let mut main_file = File::create("main.yf")?;
    let main_contents = create_main_file_contents();
    write!(main_file, "{}", main_contents)?;
    Ok(())
}

fn create_package_contents(name: String) -> std::io::Result<()> {
    create_package_file(name)?;

    debug!("Creating src/ folder");
    fs::create_dir_all("./src")?;

    env::set_current_dir("./src")?;
    create_main_file()?;
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
