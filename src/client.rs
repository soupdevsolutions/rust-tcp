use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use crate::message::Message;

pub struct Client {
    pub server_host: String,
    pub server_port: u16,
    pub stream: TcpStream,
}

impl Client {
    pub async fn connect(host: impl Into<String>, port: u16) -> anyhow::Result<Self> {
        let host = host.into();
        let address = format!("{}:{}", host, port);
        let stream = TcpStream::connect(address).await?;

        Ok(Self {
            server_host: host.into(),
            server_port: port,
            stream,
        })
    }

    pub async fn send_message(&mut self, message: Message) -> anyhow::Result<()> {
        self.stream.write_all(&message.encode()).await?;
        Ok(())
    }
}
