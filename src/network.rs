use std::collections::HashMap;
use crate::messages::message::Message;

pub struct Network {
    incoming_messages: HashMap<u32, Vec<Message>>
}

impl Network {
    pub fn send(&mut self, node_id: u32, message: Message) {
        if !self.incoming_messages.contains_key(&node_id) {
            self.incoming_messages.insert(node_id, Vec::new());
        }

        self.incoming_messages.get_mut(&node_id).unwrap().push(message);
    }

    pub fn receive(&mut self, tick: u32, node_id: u32) -> Option<Message> {
        let messages = self.incoming_messages.get_mut(&node_id);

        if let Some(messages) = messages {
            let message = messages.first_mut();

            if let Some(message) = message {
                let message_tick = match message {
                    Message::AgentMessage(message) => message.tick,
                    Message::RequestMessage(message) => message.tick,
                    Message::ResponseMessage(message) => message.tick,
                };

                if message_tick <= tick {
                    Some(messages.swap_remove(0))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn new() -> Self {
        Self {
            incoming_messages: HashMap::new()
        }
    }
}