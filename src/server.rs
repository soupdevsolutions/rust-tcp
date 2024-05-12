use tokio::io::AsyncReadExt;

use crate::message_reader::MessageReader;

pub struct Server {
    pub host: String,
    pub port: u16,
    pub running: bool,
}

impl Server {
    pub fn new(host: impl Into<String>, port: u16) -> Self {
        Server {
            host: host.into(),
            port,
            running: false,
        }
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        self.running = true;
        let listner = tokio::net::TcpListener::bind(format!("{}:{}", self.host, self.port)).await?;
        println!("Server is running on {}:{}", self.host, self.port);

        loop {
            if !self.running {
                break;
            }

            let (mut socket, addr) = listner.accept().await?;
            println!("Connection received from {}", addr);

            tokio::task::spawn(async move {
                let mut message_reader = MessageReader::new();
                loop {
                    let mut buffer = [0; 4];
                    let bytes_read = socket.read(&mut buffer).await?;
                    if bytes_read == 0 {
                        break;
                    }
                    println!("Buffer: {:?}", &buffer);

                    let message = message_reader.read(&buffer[..bytes_read]);
                    if let Some(message) = &message {
                        println!("Message: {:?}", message);
                    }
                }

                Ok::<(), anyhow::Error>(())
            });
        }
        Ok(())
    }

    pub fn stop(&mut self) {
        println!("Server is stopping on {}:{}", self.host, self.port);
        self.running = false;
    }
}
