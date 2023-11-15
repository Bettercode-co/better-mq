


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MessageType {
    Consume,
    Produce
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Header {
    time: u128,
    message_type: MessageType,
}


impl Header {
    pub fn new(time: u128, message_type: MessageType) -> Header {
        Header { time, message_type }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Message {
    header: Header,
    data: String, // slice of bytes
}

impl Message {
    pub fn new(header: Header, data: &str) -> Message {
        Message { header, data: data.to_string() }
    }
}



// tests
#[cfg(test)]
pub mod tests {
    use super::*;
    use std::time::{UNIX_EPOCH, SystemTime};

    #[test]
    fn create_message() {
        let current_time: u128 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

        println!("Current time in nanao_seconds: {}", current_time);

        let header: Header = Header::new(current_time, MessageType::Produce);
        let data: &str = "{'mamad': 10}";

        let message: Message = Message::new(header.clone(), "{'mamad': 10}");

        assert_eq!(message.header, header);
        assert_eq!(message.data, data.to_string());
    }
}