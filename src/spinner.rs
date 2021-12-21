use indicatif::{ProgressBar, ProgressStyle};

pub fn spinner(entry: String) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(120);
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&[
                "⠄", "⠆", "⠇", "⠋", "⠙", "⠸", "⠰", "⠠", "⠰", "⠸", "⠙", "⠋", "⠇", "⠆",
            ])
            .template("{spinner:.blue} {msg}"),
    );
    pb.set_message(entry);

    return pb;
}
