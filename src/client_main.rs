use rust_tcp::{client::Client, message::Message};

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let mut client = Client::connect("127.0.0.1", 8080).await?;
    for i in 0..100 {
        let message = Message::new(format!("{}", i))?;
        client.send_message(message.clone()).await?;
    }

    Ok(())
}
