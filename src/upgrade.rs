use crate::package::get_local_dir;
use indicatif::{ProgressBar, ProgressStyle};
use std::env;
use std::process::Command;

pub fn upgrade(package: &str) -> std::io::Result<()> {
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
    let package_path = format!("{}/{}", local, package);
    env::set_current_dir(package_path)?;

    Command::new("git")
        .arg("pull")
        .output()
        .expect("Failed to pull");

    pb.finish_with_message("Done ✔");

    Ok(())
}
