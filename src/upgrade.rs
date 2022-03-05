use super::spinner::spinner;
use crate::package::get_local_dir;
use std::env;
use std::process::Command;

fn git_pull() {
    Command::new("git")
        .arg("pull")
        .output()
        .expect("Failed to pull");
}

fn go_to_package_dir(name: &str) -> std::io::Result<()> {
    let local = get_local_dir();
    let package_path = format!("{}/{}", local, name);
    env::set_current_dir(package_path)?;
    Ok(())
}

pub fn upgrade(name: &str) -> std::io::Result<()> {
    let spin = spinner("Upgrading...".to_string());

    go_to_package_dir(name)?;
    git_pull();

    spin.finish_with_message("Done ✔");
    Ok(())
}
