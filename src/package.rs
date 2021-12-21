use super::debug;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InnerPackage {
    pub name: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub url: Option<String>,
    pub license: Option<String>,
    pub authors: Option<Vec<String>>,
    pub depends: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Package {
    pub package: InnerPackage,
}

pub fn fetch_remote_package(package: &str) -> Result<Package, serde_yaml::Error> {
    let uri = format!(
        "https://raw.githubusercontent.com/{}/main/package.yml",
        package
    );

    debug!("Pulling manifest from {}", uri);
    let req = reqwest::get(&uri).unwrap().text().unwrap();
    debug!("{}", req);

    let package_yaml: Package = serde_yaml::from_str(&req)?;
    debug!("{:?}", package_yaml);

    Ok(package_yaml)
}

pub fn fetch_local_package(package: &str) -> Result<Package, serde_yaml::Error> {
    let local = get_local_dir();
    let package_name = Path::new(&local).join(package).join("package.yml");

    let contents = fs::read_to_string(package_name)
        .expect("Error reading file");

    let loaded: Package = serde_yaml::from_str(&contents)?;
    Ok(loaded)
}

pub fn get_local_dir() -> String {
    match home::home_dir() {
        Some(path) => {
            let package_path = format!("{}/.local/lib/yflat/", path.display());
            debug!("{}", package_path);
            return package_path;
        }
        None => panic!("Could not get home directory"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_package_manifest_test() -> Result<(), serde_yaml::Error> {
        let pack: Package = fetch_remote_package("JakeRoggenbuck/yf-package-example")?;
        assert_eq!(pack.package.name, Some("yf-package-example".to_string()));
        assert_eq!(
            pack.package.description,
            Some("An example package for yflat".to_string())
        );
        assert_eq!(pack.package.version, Some("0.1.0".to_string()));
        assert_eq!(
            pack.package.url,
            Some("https://github.com/JakeRoggenbuck/yf-package-example".to_string())
        );
        assert_eq!(
            pack.package.authors,
            Some(vec!["Jake Roggenbuck".to_string()])
        );
        assert_eq!(pack.package.license, Some("MIT".to_string()));
        assert_eq!(pack.package.depends, None);

        Ok(())
    }
}
