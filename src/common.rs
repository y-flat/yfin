pub fn https_prefix(url: &str) -> String {
    return format!("https://{}", url);
}

pub fn github_prefix(url: &str) -> String {
    return https_prefix(format!("github.com/{}", url).as_str());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn https_prefix_test() {
        assert_eq!(https_prefix("this.com"), "https://this.com");
        assert_ne!(https_prefix("this.com"), "https://that.com");
    }

    #[test]
    fn github_prefix_test() {
        assert_eq!(github_prefix("some/thing"), "https://github.com/some/thing");
        assert_ne!(github_prefix("some/thing"), "https://that.com");
    }
}
