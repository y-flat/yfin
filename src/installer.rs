use spinners::{Spinner, Spinners};
use std::thread::sleep;
use std::time::Duration;

const YFC_URL: &str = "GitHub.com/adamhutchings/yfc";
const STDLIB_URL: &str = "GitHub.com/adamhutchings/yflib";

fn get_package_version(_package: &str) -> &str {
    // TODO: get package version from name
    return "Not 0.0.1"
}

fn get_package_url(_package: &str) -> &str {
    // TODO: get package url from name
    return "Not github.com/JakeRoggenbuck/auto-clock-speed"
}

pub fn install(url: &str) {
    let sp = Spinner::new(&Spinners::Line, format!("Installing {}...", url).into());
    sleep(Duration::from_secs(3));
    sp.stop();
    print!("\n");
}

pub fn install_compiler() {
    install(YFC_URL);
}

pub fn install_stdlib() {
    install(STDLIB_URL);
}

pub fn uninstall(package: &str) {
    let sp = Spinner::new(&Spinners::Line, format!("Uninstalling {}...", package).into());
    sleep(Duration::from_secs(3));
    sp.stop();
    print!("\n");
}

pub fn upgrade(package: &str) {
    // Get url from package name
    let package_url = get_package_url(package);
    install(package_url)
}

pub fn version(package: &str) {
    let version = get_package_version(package);
    println!("{} is version {}", package, version);
}
