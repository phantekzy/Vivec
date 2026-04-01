#[derive(Debug)]
pub enum VivecError {
    Io(std::io::Error),
    Parse(String),
    Network(String),
    InvalidUrl(String),
    Timeout,
}

impl std::error::Error for VivecError {}
