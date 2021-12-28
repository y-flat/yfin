use super::debug;
use super::warn_user;
use std::env;
use std::fs;
use std::path::Path;

pub fn check_for_local(path_name: &str) -> bool {
    match home::home_dir() {
        Some(path) => {
            let local_path =
                Path::new(format!("{}/{}/", path.display(), &path_name).as_str()).to_owned();

            if local_path.exists() {
                return true;
            } else {
                debug!("Could not find ~/{}", path_name);
            }
        }
        None => println!("Could not get home directory"),
    }

    return false;
}

pub fn create_local(path_name: &str) {
    match home::home_dir() {
        Some(path) => {
            let local_path =
                Path::new(&format!("{}/{}", path.display(), &path_name).as_str()).to_owned();

            match fs::create_dir_all(local_path.clone()) {
                Ok(_) => debug!("Created {:?}", local_path),
                Err(e) => eprintln!("{}", e),
            }
        }
        None => println!("Could not get home directory"),
    }
}

pub fn check_for_local_lib() -> bool {
    check_for_local(".local/lib/yflat/")
}

pub fn create_local_lib() {
    create_local(".local/lib/yflat");
}

pub fn check_for_local_bin() -> bool {
    check_for_local(".yflat/bin/")
}

pub fn create_local_bin() {
    create_local(".yflat/bin/")
}

pub fn check_for_bin_in_path() -> bool {
    if let Ok(path) = env::var("PATH") {
        if path.contains(".yflat/bin") {
            return true;
        }
    }

    match home::home_dir() {
        Some(path) => {
            let local_path =
                Path::new(format!("{}/{}", path.display(), ".yflat/bin").as_str()).to_owned();

            match fs::create_dir_all(local_path.clone()) {
                Ok(_) => warn_user!(format!(
                    "Make sure to add {:?} to PATH to be able to run compiler binary",
                    local_path,
                )),
                Err(e) => eprintln!("{}", e),
            }
        }
        None => println!("Could not get home directory"),
    }
    return false;
}
