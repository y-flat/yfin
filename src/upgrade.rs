use crate::install::install;
use crate::package::get_package_url;

pub fn upgrade(package: &str) {
    // Get url from package name
    let package_url = get_package_url(package);
    install(package_url)
}
