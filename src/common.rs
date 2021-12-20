pub fn https_prefix(url: &str) -> String {
    return format!("https://{}", url);
}

pub fn github_prefix(url: &str) -> String {
    return https_prefix(format!("github.com/{}", url).as_str());
}
