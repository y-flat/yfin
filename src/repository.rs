use super::debug;
use super::error;
use super::package::get_local_dir;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::Path;

pub fn update_repository_cache() -> Result<(), error::Error> {
    let uri = format!(
        "https://raw.githubusercontent.com/Camerooooon/yfin-official-repository/master/packages"
    );
    let mut req = reqwest::get(&uri).unwrap();

    if !req.status().is_success() {
        return Err(error::Error::RequestFailed);
    }

    let text = req.text().unwrap();

    debug!("{}", text);

    let local = get_local_dir();
    let path = Path::new(&local).join("packages");
    println!("{:?}", path);

    match std::fs::File::create(path) {
        Ok(mut reference) => match reference.write_all(text.as_bytes()) {
            Ok(_) => return Ok(()),
            Err(_) => {
                return Err(error::Error::FileSystemError);
            }
        },
        Err(e) => panic!("{}", e),
    };
}

pub fn get_official_repositories() -> Result<HashMap<String, String>, error::Error> {
    let local = get_local_dir();
    let path = Path::new(&local).join("packages");

    if !path.exists() {
        println!("You have not downloaded the official repository list, some packages may be missing. Download it with yfin update\n\n");
    }

    match std::fs::File::open(path) {
        Ok(mut file) => {
            let mut text = String::new();
            file.read_to_string(&mut text);
            let mut package: HashMap<String, String> = HashMap::new();
            for line in text.lines() {
                let vals: Vec<&str> = line.trim().split(":").collect();
                package.insert(vals[1].to_string(), vals[0].to_string());
            }
            Ok(package)
        }
        Err(_) => return Err(error::Error::FileSystemError),
    }
}

pub fn get_official_url(name: &str) -> Result<String, error::Error> {
    match get_official_repositories() {
        Ok(repository) => {
            let package = repository.get(name);
            if package == None {
                return Err(error::Error::PackageNotFoundError);
            }
            return Ok(package.unwrap().to_string());
        }
        Err(_) => return Err(error::Error::FileSystemError),
    }
}
