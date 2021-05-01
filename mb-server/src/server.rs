use std::io;

pub trait Server {
    fn init(&self) -> io::Result<()>;
    fn start(&self) -> io::Result<()>;
}
