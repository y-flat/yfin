use super::debug;
use super::error::Error;
use super::package::get_local_dir;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;

const YFIN_OFFICIAL_REPOSITORY: &str =
    "https://raw.githubusercontent.com/Camerooooon/yfin-official-repository/master/packages";

pub fn update_repository_cache() -> Result<(), io::Error> {
    let mut req = match reqwest::get(YFIN_OFFICIAL_REPOSITORY) {
        Ok(r) => r,
        Err(..) => std::process::exit(1),
    };

    let text = req.text().unwrap();

    let local = get_local_dir();
    let path = Path::new(&local).join("packages");
    debug!("{:?}", path);

    let mut reference = File::create(path)?;
    reference.write_all(text.as_bytes())?;

    Ok(())
}

pub fn get_official_repositories() -> Result<HashMap<String, String>, io::Error> {
    let local = get_local_dir();
    let path = Path::new(&local).join("packages");

    if !path.exists() {
        println!(
            "You have not downloaded the official repository list, some packages may be missing.\
Download it with yfin update\n\n"
        );
    }

    let mut file = File::open(path)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;

    let mut package: HashMap<String, String> = HashMap::new();

    for line in text.lines() {
        let vals: Vec<&str> = line.trim().split(":").collect();
        package.insert(vals[1].to_string(), vals[0].to_string());
    }

    Ok(package)
}

pub fn get_official_url(name: &str) -> Result<String, Error> {
    match get_official_repositories() {
        Ok(repository) => {
            let package = repository.get(name);
            if package == None {
                return Err(Error::PackageNotFoundError);
            }
            return Ok(package.unwrap().to_string());
        }
        Err(_) => return Err(Error::FileSystemError),
    }
}
