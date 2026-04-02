pub struct ParsedUrl {
    pub host: String,
    pub port: String,
    pub path: String,
}

pub fn parse_url(url: &str) -> Result<ParsedUrl, String> {
    let remainder = url.strip_prefix("http://").ok_or("Only Http supported")?;
    let (host_port, path_query) = remainder.split_once("/").unwrap_or((remainder, ""));
}
