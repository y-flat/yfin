use crate::package::get_local_dir;
use std::env;
use std::process::Command;

pub fn upgrade(package: &str) -> std::io::Result<()> {
    let local = get_local_dir();
    let package_path = format!("{}/{}", local, package);
    env::set_current_dir(package_path)?;

    Command::new("git")
        .arg("pull")
        .output()
        .expect("Failed to pull");

    Ok(())
}
