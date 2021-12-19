use spinners::{Spinner, Spinners};
use std::thread::sleep;
use std::time::Duration;

const YFC_URL: &str = "GitHub.com/adamhutchings/yfc";
const STDLIB_URL: &str = "GitHub.com/adamhutchings/yflib";

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
