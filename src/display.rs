#[macro_export]
macro_rules! bold_blue_word {
    ($a:expr) => {{
        format!(
            "{}{}{}{}{}",
            termion::style::Bold,
            termion::color::Fg(termion::color::Blue),
            $a,
            termion::color::Fg(termion::color::Reset),
            termion::style::Reset,
        )
    }};
}
