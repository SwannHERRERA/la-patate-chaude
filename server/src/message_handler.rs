use log::{debug, info, trace};

use shared::message::{Message, SubscribeError, SubscribeResult};

#[derive(Debug)]
pub struct MessageHandler {
    players: Vec<String>,
}

impl MessageHandler {
    pub fn new(players: Vec<String>) -> MessageHandler {
        MessageHandler { players }
    }

    pub fn handle_message(&mut self, message: Message) -> Message {
        info!("Incomming Message: {:?}", message);
        match message {
            Message::Hello => self.handle_hello(),
            Message::Subscribe { name } => self.handle_subscribtion(name),
            _ => panic!("Not implemented")
        }
    }

    fn handle_subscribtion(&mut self, name: String) -> Message {
        let answer = if self.players.contains(&name) {
            Message::SubscribeResult(SubscribeResult::Err(SubscribeError::AlreadyRegistered))
        } else {
            Message::SubscribeResult(SubscribeResult::Ok)
        };
        self.players.push(name);
        debug!("Answer: {:?}", answer);
        trace!("Players: {:?}", self.players);
        answer
    }

    fn handle_hello(&self) -> Message {
        let answer = Message::Welcome { version: 1 };
        debug!("Answer: {:?}", answer);
        answer
    }
}


#[cfg(test)]
mod tests {
    use std::sync::Once;

    use super::*;

    static INIT: Once = Once::new();

    pub fn setup() -> () {
        INIT.call_once(|| {
            std::env::set_var("RUST_LOG", "trace");
        });
    }

    #[test]
    fn test_handle_hello() {
        setup();
        let mut handler = MessageHandler::new(vec![]);
        println!("{:?}", handler);
        let message = Message::Hello;
        let answer: Message = handler.handle_message(message);
        debug!("Answer: {:?}", answer);
        assert!(matches!(answer, Message::Welcome { version: 1 }));
    }

    #[test]
    fn test_handle_subscribe() {
        setup();
        let mut handler = MessageHandler::new(vec![]);
        let message = Message::Subscribe { name: "John".to_owned() };
        let answer: Message = handler.handle_message(message);
        println!("Answer: {:?}", answer);
        assert!(matches!(answer, Message::SubscribeResult(SubscribeResult::Ok)));
    }

    #[test]
    fn test_handle_subscribe_already_registered() {
        setup();
        let mut handler = MessageHandler::new(vec!["John".to_owned()]);
        let answer = handler.handle_subscribtion("John".to_owned());
        assert!(matches!(answer, Message::SubscribeResult(SubscribeResult::Err(SubscribeError::AlreadyRegistered))));
    }
}