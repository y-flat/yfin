use super::debug;
use super::package::get_local_dir;
use super::spinner::spinner;
use std::fs;

pub fn uninstall(package: &str) -> std::io::Result<()> {
    let spin = spinner("Uninstalling...".to_string());

    let local = get_local_dir();
    fs::remove_dir_all(format!("{}/{}", local, package))?;
    debug!("Successfully removed package");

    spin.finish_with_message("Done ✔");

    Ok(())
}
