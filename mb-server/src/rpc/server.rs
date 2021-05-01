use crate::options::Options;
use crate::server::Server;
use std::io;

#[derive(Debug, Clone)]
pub struct RpcServer {
    options: Options,
}

impl RpcServer {
    pub fn new(opts: Option<Options>) -> Self {
        // TODO: fix muttable
        let opt = match opts {
            Some(opt) => opt,
            None => Options::default(),
        };

        RpcServer { options: opt }
    }
}

impl Server for RpcServer {
    fn options(&self) -> &Options {
        &self.options
    }

    fn init(&self) -> io::Result<()> {
        println!("Init server");
        Ok(())
    }

    fn start(&self) -> io::Result<()> {
        println!("Start server");
        Ok(())
    }
}
