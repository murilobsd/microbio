/// Server
use std::io::Result;
use tokio::net::{TcpListener};

#[derive(Debug)]
pub struct ServerOptions {
    pub name: String,
    pub address: String,
    pub id: String,
}

impl Default for ServerOptions {
    fn default() -> Self {
        Self {
            name: "microbio-server".to_string(),
            address: "127.0.0.1:8081".to_string(),
            id: "140a5437-7942-48b0-aaf8-c14a783677cd".to_string()
        }
    }
}

#[derive(Debug)]
pub struct RpcServer {
    pub options: ServerOptions
}

impl RpcServer {
    /// Create new rpc server
    pub fn new() -> Self {
        Self{
            options: ServerOptions::default(),
        }
    }

    /// Initialise rpc server (optins)
    pub fn init(&self, _opt: Option<ServerOptions>) -> Result<()> {
        Ok(())
    }

    pub async fn start(&self) -> Result<()> {
        let listener = TcpListener::bind(&self.options.address).await?;
        loop {
            let (socket, _) = listener.accept().await?;
            println!("Connectado {:?}", socket);
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_server() {
        RpcServer::new();
        assert_eq!(2 + 2, 4);
    }
}
