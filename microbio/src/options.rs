use mb_server::{rpc::server::RpcServer, server::Server};

pub struct Options {
    pub server: Box<dyn Server>,
}

impl Options {
    pub fn new() -> Self {
        Self {
            server: Box::new(RpcServer::new(None)),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn default_options() {
        assert_eq!(1 + 1, 2);
    }
}
