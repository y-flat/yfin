use yaml_rust::{Yaml, YamlLoader};

pub fn get_package_version(_package: &str) -> &str {
    // TODO: get package version from name
    return "Not 0.0.1";
}

pub fn get_package_url(_package: &str) -> &str {
    // TODO: get package url from name
    return "Not github.com/JakeRoggenbuck/auto-clock-speed";
}

// Convert this to object oriented at some point
pub fn fetch_package_manifest(_package: &str) -> Result<Vec<Yaml>, ()> {
    let uri = format!(
        "https://raw.githubusercontent.com/{}/main/package.yml",
        _package
    );

    let req = reqwest::get(&uri).unwrap().text();

    Ok(YamlLoader::load_from_str(&req.expect("Couldn't get manifest")).unwrap())
}
