use std::time::{UNIX_EPOCH, SystemTime};


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MessageType {
    Consume,
    Produce,
    Response,
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

    pub fn new_current_time(message_type: MessageType) -> Header {
        let current_time: u128 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

        Header { time: current_time, message_type }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Message {
    header: Header,
    topic: String,
    data: String, // slice of bytes
}

impl Message {
    pub fn new(header: Header, data: &str, topic: &str) -> Message {
        Message { header, data: data.to_string(), topic: topic.to_string() }
    }

    pub fn get_data(&self) -> &str {
        &self.data
    }

    pub fn get_header(&self) -> Header {
        self.header
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
        let topic: &str = "chat1";

        let message: Message = Message::new(header.clone(), data, topic);

        assert_eq!(message.header, header);
        assert_eq!(message.data, data.to_string());
        assert_eq!(message.topic, topic.to_string());
    }
}