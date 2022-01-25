use super::debug;
use super::package::get_local_dir;
use super::spinner::spinner;
use std::fs;
use std::path::Path;
use nix::unistd::Uid;

pub fn uninstall(package: &str) -> std::io::Result<()> {
    let spin = spinner("Uninstalling...".to_string());

    let local = get_local_dir();
    let package_name = format!("{}/{}", local, package);

    if !Path::new(&package_name).exists() {
        eprintln!("Package {} does not exist", package,);
        std::process::exit(0);
    }

    // Check for root before running remove_dir_all
    if !Uid::effective().is_root() {
        fs::remove_dir_all(package_name)?;
        debug!("Successfully removed package");
        spin.finish_with_message("Done ✔");
    } else {
        spin.finish_with_message("Please do not run uninstall with root.")
    }

    Ok(())
}
