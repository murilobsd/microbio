use bytes::BufMut;
use std::collections::HashMap;
use std::io::Result;

pub trait Transport {
    fn listen(&self, addr: String) -> Result<Box<dyn Client>>;
}

pub struct Message {
    pub header: HashMap<String, String>,
    pub body: dyn BufMut,
}

pub trait Socket {
    fn recv(&self, msg: &mut Message) -> Result<()>;
    fn send(&self, msg: &mut Message) -> Result<()>;
    fn close(&self) -> Result<()>;
    fn local(&self) -> String;
    fn remote(&self) -> String;
}

pub trait Client: Socket {}

pub trait Listener {
    fn addr(&self) -> String;
    fn close(&self) -> Result<()>;
    fn accept(&self, f: dyn Fn(dyn Socket)) -> Result<()>;
}
