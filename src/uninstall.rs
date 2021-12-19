use spinners::{Spinner, Spinners};
use std::thread::sleep;
use std::time::Duration;

pub fn uninstall(package: &str) {
    let sp = Spinner::new(
        &Spinners::Line,
        format!("Uninstalling {}...", package).into(),
    );
    sleep(Duration::from_secs(3));
    sp.stop();
    print!("\n");
}
