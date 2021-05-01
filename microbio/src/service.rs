use crate::options::Options;
use std::{fmt, io, thread, time};

/// Trait service
pub trait Service: fmt::Display {
    /// The service name
    fn name(&self) -> &'static str;
    /// The service init
    fn init(&self);
    /// The service run
    fn run(&self) -> io::Result<()>;
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
        // sleep
        loop {
            thread::sleep(time::Duration::from_secs(1));
            self.options.server.start()?;
        }
        #[allow(unreachable_code)]
        Ok(())
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
