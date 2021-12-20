use super::debug;
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

    match home::home_dir() {
        Some(path) => {
            let package_path = format!("{}/.local/lib/yflat/{}/", path.display(), package);
            debug!("{}", package_path);
            fs::remove_dir_all(package_path)?;
        }
        None => println!("Could not get home directory"),
    }
    pb.finish_with_message("Done ✔");

    Ok(())
}
