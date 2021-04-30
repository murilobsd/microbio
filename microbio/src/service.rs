use std::{io, thread, time};

/// Trait service
pub trait Service {
    /// The service name
    fn name(&self) -> &'static str;
    /// The service init
    fn init(&self);
    /// The service run
    fn run(&self) -> io::Result<()>;
}

struct MicrobioService {}

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
        };
        Ok(())
    }
}

pub fn new_service() -> Box<dyn Service> {
    Box::new(MicrobioService {})
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}