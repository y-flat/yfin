use super::debug;
use crate::common::github_prefix;
use crate::package::fetch_package_manifest;
use git2::Repository;
use indicatif::{ProgressBar, ProgressStyle};
use super::package::{Package, get_local_dir};

const YFC_URL: &str = "adamhutchings/yfc";
const YFLIB_URL: &str = "adamhutchings/yflib";

pub fn install(url: &str) -> Result<(), serde_yaml::Error> {
    let pack: Package = fetch_package_manifest(url)?;

    // Checks for the needed parts of the manifest
    if pack.package.name == None {
        debug!("Could not find field name in package, repository does not exist at the given url");
        return Ok(println!("Package does not exist at the given url"));
    }

    println!("Found a package with name {}", pack.package.name.clone().unwrap());

    // Starts a spinner that will finish when the git repository is cloned
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(120);
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&[
                "⠄", "⠆", "⠇", "⠋", "⠙", "⠸", "⠰", "⠠", "⠰", "⠸", "⠙", "⠋", "⠇", "⠆",
            ])
            .template("{spinner:.blue} {msg}"),
    );
    pb.set_message("Installing...");

    let https_url = github_prefix(url);
    let local = get_local_dir();
    // Clone the repo to /home/<user>/.local/lib/yflat/<name>
    let _repo = match Repository::clone(
        &https_url,
        format!("{}/{}/", local, pack.package.name.clone().unwrap()),
    ) {
        Ok(repo) => debug!("{:#?}", repo.path()),
        Err(e) => panic!("failed to clone: {}", e),
    };

    pb.finish_with_message("Done ✔");

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
