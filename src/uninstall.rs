use super::debug;
use super::package::get_local_dir;
use super::spinner::spinner;
use std::fs;
use std::path::Path;

fn check_if_package_exists(package_path: &str, package: &str) {
    if !Path::new(&package_path).exists() {
        eprintln!("Package {} does not exist", package);
        std::process::exit(0);
    }
}

pub fn uninstall(package: &str) -> std::io::Result<()> {
    let spin = spinner("Uninstalling...".to_string());

    let package_path = format!("{}/{}", get_local_dir(), package);
    check_if_package_exists(&package_path, package);

    fs::remove_dir_all(package_path)?;
    debug!("Successfully removed package");
    spin.finish_with_message("Done ✔");

    Ok(())
}
