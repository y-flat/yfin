use super::debug;
use std::fs;
use std::path::Path;

pub fn check_for_local() -> bool {
    match home::home_dir() {
        Some(path) => {
            let local_path =
                Path::new(format!("{}/.local/lib/yflat/", path.display()).as_str()).to_owned();

            if local_path.exists() {
                return true;
            } else {
                debug!("Could not find ~/.local/lib/yflat/");
            }
        }
        None => println!("Could not get home directory"),
    }

    return false;
}

pub fn create_local() {
    match home::home_dir() {
        Some(path) => {
            let local_path =
                Path::new(format!("{}/.local/lib/yflat/", path.display()).as_str()).to_owned();

            match fs::create_dir_all(local_path.clone()) {
                Ok(_) => debug!("Created {:?}", local_path),
                Err(e) => eprintln!("{}", e),
            }
        }
        None => println!("Could not get home directory"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_for_local_test() {
        assert!(check_for_local());
    }
}
