use tokio::io::AsyncReadExt;

use crate::message_reader::MessageReader;

#[derive(Debug, Clone)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

impl Server {
    pub fn new(host: impl Into<String>, port: u16) -> Self {
        Server {
            host: host.into(),
            port,
        }
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let listner = tokio::net::TcpListener::bind(format!("{}:{}", self.host, self.port)).await?;
        println!("Server is running on {}:{}", self.host, self.port);

        loop {
            let (mut socket, addr) = listner.accept().await?;
            println!("Connection received from {}", addr);

            tokio::task::spawn(async move {
                let mut message_reader = MessageReader::new();
                'handler: loop {
                    let mut buffer = [0; 256];
                    let bytes_read = socket.read(&mut buffer).await?;

                    let messages = message_reader.read(&buffer[..bytes_read])?;
                    for message in messages {
                        if message.content == "exit" {
                            println!("Connection closed by client");
                            break 'handler;
                        }
                        println!("Message: {:?}", message);
                    }
                }
                Ok::<(), anyhow::Error>(())
            });
        }
        Ok(())
    }
}
