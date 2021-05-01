use crate::options::Options;
use crate::server::Server;
use std::io::{Read, Result, Write};
use std::{net, thread};

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

    fn handle_client(&self, mut stream: net::TcpStream) {
        let mut data = [0_u8; 50];
        while match stream.read(&mut data) {
            Ok(size) => {
                stream.write_all(&data[0..size]).unwrap();
                true
            }
            Err(_) => {
                println!("An error ocurred, terminating connection");
                stream.shutdown(net::Shutdown::Both).unwrap();
                false
            }
        } {}
    }
}

impl Server for RpcServer {
    fn options(&self) -> &Options {
        &self.options
    }

    fn init(&self) -> Result<()> {
        println!("Init server");
        Ok(())
    }

    fn start(&self) -> Result<()> {
        println!("Server listening {:?}", self.options.address());
        let listener = net::TcpListener::bind(self.options.address()).unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    let s = self.clone();
                    thread::spawn(move || {
                        // connection ok
                        s.handle_client(stream)
                    });
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
        // close the socket server
        drop(listener);
        Ok(())
    }
}
