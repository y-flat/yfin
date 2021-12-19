use crate::package::fetch_package_manifest;
use indicatif::{ProgressBar, ProgressStyle};
use std::thread::sleep;
use std::time::Duration;

const YFC_URL: &str = "GitHub.com/adamhutchings/yfc";
const STDLIB_URL: &str = "GitHub.com/adamhutchings/yflib";

pub fn install(url: &str) {
    let pb = ProgressBar::new(6942);
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{msg:.green} {bar} ({bytes} of {total_bytes}) (about {eta})"),
    );
    pb.set_message("Fetching Manifest");
    for _ in 0..pb.length() {
        sleep(Duration::from_millis(1));
        pb.inc(1);
    }

    match fetch_package_manifest(url) {
        Ok(manifest) => {
            pb.finish_with_message("Done ✔");
            if manifest[0]["package"]["name"].as_str() == None {
                return println!("Github repository does not exist");
            }
            println!(
                "Found a package with name {}",
                manifest[0]["package"]["name"].as_str().unwrap()
            );
        }
        Err(_) => pb.abandon_with_message("Failed to fetch package manifest"),
    }
}

pub fn install_compiler() {
    install(YFC_URL);
}

pub fn install_stdlib() {
    install(STDLIB_URL);
}
