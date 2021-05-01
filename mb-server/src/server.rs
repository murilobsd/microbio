use crate::options::Options;
use std::io;

pub trait Server {
    fn options(&self) -> &Options;
    fn init(&self) -> io::Result<()>;
    fn start(&self) -> io::Result<()>;
}
