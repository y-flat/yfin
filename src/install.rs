use super::bold_color_text;
use super::debug;
use super::package::{get_local_dir, Package};
use super::repository;
use super::spinner::spinner;
use crate::common::github_prefix;
use crate::package::fetch_remote_package;
use git2::Repository;
use std::path::Path;
use termion::color;

const YFC_URL: &str = "adamhutchings/yfc";
const YFLIB_URL: &str = "adamhutchings/yflib";

pub fn install(mut name: &str) -> Result<(), serde_yaml::Error> {
    let spin = spinner("Installing...".to_string());

    let mut url: String = name.to_string();

    match repository::get_official_url(&url) {
        Ok(pkg) => url = pkg.to_string(),
        Err(_) => {}
    }

    let pack: Package = match fetch_remote_package(&url) {
        Err(error) => {
            spin.finish_with_message(format!("{}", error));
            return Ok(());
        }
        Ok(package) => package,
    };

    // Checks for the needed parts of the manifest
    if pack.package.name == None {
        debug!("Could not find field name in package, repository does not exist at the given url");
        return Ok(println!("Package does not exist at the given url"));
    }

    println!(
        "Found a package with name {}",
        bold_color_text!(pack.package.name.clone().unwrap(), color::Blue)
    );

    let https_url = github_prefix(&url);
    let local = get_local_dir();
    let package_name = Path::new(&local).join(pack.package.name.clone().unwrap());
    debug!("{}", package_name.display());

    if Path::new(&package_name).exists() {
        eprintln!(
            "Package already exists.\nTry {}",
            bold_color_text!(
                format!("yfin upgrade {}", pack.package.name.clone().unwrap()),
                color::Blue
            )
        );
        std::process::exit(0);
    }

    // Clone the repo to /home/<user>/.local/lib/yflat/<name>
    let _repo = match Repository::clone(&https_url, package_name) {
        Ok(repo) => debug!("{:#?}", repo.path()),
        Err(e) => panic!("failed to clone: {}", e),
    };

    spin.finish_with_message("Done ✔");

    Ok(())
}

pub fn install_compiler() {
    install(YFC_URL).unwrap();
}

pub fn install_yflib() {
    install(YFLIB_URL).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn const_test() {
        // Making sure the urls lead to something with the correct name
        // In case we change locations later
        assert!(YFC_URL.contains("yfc"));
        assert!(YFLIB_URL.contains("yflib"));
    }
}
