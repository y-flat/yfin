use spinners::{Spinner, Spinners};
use std::time::Duration;
use std::thread::sleep;

pub fn uninstall(package: &str) {
    let sp = Spinner::new(&Spinners::Line, format!("Uninstalling {}...", package).into());
    sleep(Duration::from_secs(3));
    sp.stop();
    print!("\n");
}