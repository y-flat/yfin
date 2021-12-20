use std::path::Path;

pub fn check_for_local() -> bool {
    match home::home_dir() {
        Some(path) => {
            let local_path = Path::new(format!("{}/.local/lib/yflat/", path.display()).as_str()).to_owned();

            if local_path.exists() {
                return true;
            }
        }
        None => println!("Could not get home directory"),
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_for_local_test() {
        assert!(check_for_local());
    }
}
