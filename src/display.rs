#[macro_export]
macro_rules! bold_color_text {
    ($a:expr, $b:expr) => {{
        format!(
            "{}{}{}{}{}",
            termion::style::Bold,
            termion::color::Fg($b),
            $a,
            termion::color::Fg(termion::color::Reset),
            termion::style::Reset,
        )
    }};
}

#[macro_export]
macro_rules! warn_user {
    ($a:expr) => {{
        println!(
            "{}{}WARN:{}{} {}",
            termion::style::Bold,
            termion::color::Fg(termion::color::Yellow),
            termion::color::Fg(termion::color::Reset),
            termion::style::Reset,
            $a,
        );
    }};
}
