use rust_tcp::{client::Client, message::Message};

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let mut client = Client::connect("127.0.0.1", 8080).await?;
    for i in 1..11 {
        let message = Message::new(format!("Hello, world x {}!", i));
        client.send_message(message.clone()).await?;
    } 

    client.send_message(Message::new("exit")).await?;

    Ok(())
}
