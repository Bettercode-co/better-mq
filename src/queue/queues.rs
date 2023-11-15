#![warn(missing_docs)]

/// Defines methods that would be expected on a queue data structure
pub trait IsQueue<T: Clone> {
    /// Adds a new value to a queue
    fn add(&mut self, val: T) -> Result<Option<T>, &str>;

    /// Removes an element from the queue and returns it
    fn remove(&mut self) -> Result<T, &str>;

    /// Peek at the head of the queue
    fn peek(&self) -> Result<T, &str>;

    /// Gets the size of the queue
    fn size(&self) -> usize;
}


#[derive(Debug)]
pub struct Queue<T: Clone> {
    queue: Vec<T>,
}

impl<T: Clone> Queue<T> {
    /// Create a new queue
    pub fn new() -> Queue<T> {
        Queue { queue: vec![] }
    }
}

impl<T: Clone> Default for Queue<T> {

    fn default() -> Queue<T> {
        Queue { queue: vec![] }
    }
}

impl<T: Clone> IsQueue<T> for Queue<T> {

    fn add(&mut self, val: T) -> Result<Option<T>, &str> {
        self.queue.push(val);
        Ok(None)
    }

    fn remove(&mut self) -> Result<T, &str> {
        if !self.queue.is_empty() {
            Ok(self.queue.remove(0usize))
        } else {
            Err("The queue is empty")
        }
    }


    fn peek(&self) -> Result<T, &str> {
        match self.queue.first() {
            Some(val) => Ok(val.clone()),
            None => Err("The Queue is empty"),
        }
    }


    fn size(&self) -> usize {
        self.queue.len()
    }
}

/// Creates a new `Queue<T>`
#[macro_export]
macro_rules! queue {
    () => { Queue::new() };
    ($($x:expr),+) => {
        {
            let mut temp_q = Queue::default();
            $(
                let _ = temp_q.add($x);
            )*
            temp_q
        }
    };
}


#[derive(Debug)]
pub struct CircularBuffer<T: Clone> {
    queue: Vec<T>,
    capacity: usize,
    default_value: Option<T>,
}

impl<T: Clone> CircularBuffer<T> {
    pub fn new(capacity: usize) -> CircularBuffer<T> {
        CircularBuffer {
            queue: vec![],
            capacity,
            default_value: None,
        }
    }

    pub fn with_default(capacity: usize, default_value: T) -> CircularBuffer<T> {
        let queue = vec![default_value.clone(); capacity];

        CircularBuffer {
            queue,
            capacity,
            default_value: Some(default_value),
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

impl<T: Clone> IsQueue<T> for CircularBuffer<T> {
    /// Adds an element to a circular buffer
    fn add(&mut self, val: T) -> Result<Option<T>, &str> {
        if self.queue.len() < self.capacity {
            self.queue.push(val);
            Ok(None)
        } else {
            self.queue.push(val);
            Ok(Some(self.queue.remove(0usize)))
        }
    }

    fn remove(&mut self) -> Result<T, &str> {
        if !self.queue.is_empty() {
            if let Some(val) = self.default_value.clone() {
                self.queue.push(val);
            };
            Ok(self.queue.remove(0usize))
        } else {
            Err("The Buffer is empty")
        }
    }

    fn peek(&self) -> Result<T, &str> {
        match self.queue.first() {
            Some(val) => Ok(val.clone()),
            None => Err("The Queue is empty"),
        }
    }

    fn size(&self) -> usize {
        self.queue.len()
    }
}



#[cfg(test)]
pub mod tests {
    use crate::types::message::{MessageType, Header, Message};
    use super::*;
    

    #[test]
    fn test_queue() {
        let mut q : Queue<Message> = queue![];

        q.add(new_message(1)).unwrap();
        q.add(new_message(2)).unwrap();

        let msg = q.remove().unwrap();
        assert_eq!(msg.get_data(), "1");

        let msg = q.remove().unwrap();
        assert_eq!(msg.get_data(), "2");
        
    }

    #[test]
    fn test_circular_queue() {
        let mut q : CircularBuffer<Message> = CircularBuffer::new(10);

        q.add(new_message(1)).unwrap();
        q.add(new_message(2)).unwrap();

        let msg = q.remove().unwrap();
        assert_eq!(msg.get_data(), "1");

        let msg = q.remove().unwrap();
        assert_eq!(msg.get_data(), "2");
        
    }


    fn new_message(id: i32) -> Message {
        let h: Header = Header::new_current_time(MessageType::Consume);
        Message::new(h, &format!("{}", id), "topic")
    }
}