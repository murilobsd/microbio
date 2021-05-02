use std::time::Duration;

/// Transport options
#[derive(Debug, Clone)]
pub struct Options {
    pub addrs: Vec<String>,
    pub timeout: Duration,
}
