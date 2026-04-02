pub struct ParsedUrl {
    pub host: String,
    pub port: String,
    pub path: String,
}

pub fn parse_url(url: &str) -> Result<ParsedUrl, String> {}
