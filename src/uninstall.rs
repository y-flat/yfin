use super::debug;
use super::package::get_local_dir;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;

pub fn uninstall(package: &str) -> std::io::Result<()> {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(120);
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&[
                "⠄", "⠆", "⠇", "⠋", "⠙", "⠸", "⠰", "⠠", "⠰", "⠸", "⠙", "⠋", "⠇", "⠆",
            ])
            .template("{spinner:.blue} {msg}"),
    );
    pb.set_message("Uninstalling...");

    let local = get_local_dir();
    fs::remove_dir_all(format!("{}/{}", local, package))?;
    debug!("Successfully removed package");

    pb.finish_with_message("Done ✔");

    Ok(())
}
