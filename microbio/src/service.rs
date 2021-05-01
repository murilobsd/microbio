use std::{fmt, io};

use crate::options::Options;

/// Trait service
pub trait Service: fmt::Display {
    /// The service name
    fn name(&self) -> &'static str;

    /// The service init
    fn init(&self);

    /// The service run
    fn run(&self) -> io::Result<()>;

    /// The service options
    fn options(&self) -> &Options;
}

struct MicrobioService {
    options: Options,
}

impl MicrobioService {
    pub fn new() -> Self {
        Self {
            options: Options::new(),
        }
    }
}

impl Service for MicrobioService {
    fn name(&self) -> &'static str {
        "microbio-service"
    }

    fn init(&self) {
        println!("Init service: {}", self.name());
    }

    // TODO: remove example run server
    fn run(&self) -> io::Result<()> {
        println!("Run service: {}", self.name());
        self.options.server.start()?;
        Ok(())
    }

    fn options(&self) -> &Options {
        &self.options
    }
}

impl fmt::Display for MicrobioService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

pub fn new_service() -> Box<dyn Service> {
    Box::new(MicrobioService::new())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
