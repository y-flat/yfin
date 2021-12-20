use super::debug;
use crate::common::github_prefix;
use crate::package::fetch_package_manifest;
use git2::Repository;
use indicatif::{ProgressBar, ProgressStyle};

const YFC_URL: &str = "adamhutchings/yfc";
const YFLIB_URL: &str = "adamhutchings/yflib";

pub fn install(url: &str) -> Result<(), ()> {
    let manifest = fetch_package_manifest(url)?;

    // Checks for the needed parts of the manifest
    if manifest[0]["package"]["name"].as_str() == None {
        debug!("Could not find field name in package, repository does not exist at the given url");
        return Ok(println!("Package does not exist at the given url"));
    }

    let name = manifest[0]["package"]["name"].as_str().unwrap();
    println!("Found a package with name {}", name);

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

    match home::home_dir() {
        Some(path) => {
            let https_url = github_prefix(url);
            // Clone the repo to /home/<user>/.local/lib/yflat/<name>
            let _repo = match Repository::clone(
                &https_url,
                format!("{}/.local/lib/yflat/{}/", path.display(), name),
            ) {
                Ok(repo) => debug!("{:#?}", repo.path()),
                Err(e) => panic!("failed to clone: {}", e),
            };
        }
        None => println!("Impossible to get your home dir!"),
    }

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
