use crate::{constants::METADATA_SIZE, message::Message};

pub struct MessageReader {
    pub buffer: Vec<u8>,
}

impl MessageReader {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    pub fn can_parse(&self) -> bool {
        if self.buffer.len() < METADATA_SIZE {
            return false;
        }
        let length = u16::from_be_bytes([self.buffer[1], self.buffer[2]]);
        self.buffer.len() >= METADATA_SIZE + length as usize
    }

    pub fn read(&mut self, data: &[u8]) -> Option<Message> {
        self.buffer.extend_from_slice(data);
        if self.can_parse() {
            let message = self.flush().ok()?;
            return Some(message);
        }

        None
    }

    pub fn flush(&mut self) -> anyhow::Result<Message> {
        let length = u16::from_be_bytes([self.buffer[1], self.buffer[2]]);
        let message_length = METADATA_SIZE + length as usize;
        let message = self.buffer[..message_length].to_vec();
        self.buffer = self.buffer[message_length..].to_vec();

        Message::decode(&message)
    }
}

impl Default for MessageReader {
    fn default() -> Self {
        Self::new()
    }
}
