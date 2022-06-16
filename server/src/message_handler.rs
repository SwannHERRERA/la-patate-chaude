use log::{info, debug};
use shared::message::{Message, SubscribeResult};
struct MessageHandler {
  player: Vec<String>,
}

impl MessageHandler {
  pub fn new() -> MessageHandler {
    MessageHandler {
      player: Vec::new(),
    }
  }

  pub fn handle_message(&self, message: Message) -> Message {
      info!("Hello: {:?}", message);
      match message {
        Message::Hello => self.handle_hello(),
        Message::Subscribe { name } => self.handle_subscribtion(name),
        _ => panic!("Not implemented")
      }
  }

  fn handle_subscribtion(&self, name: String) -> Message {
    let answer = Message::SubscribeResult(SubscribeResult::Ok);
    self.player.push(name);
    debug!("Answer: {:?}", answer);
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

    use super::*;
    use std::sync::Once;

    static INIT: Once = Once::new();

    pub fn setup() -> () {
      INIT.call_once(|| {
        std::env::set_var("RUST_LOG", "trace");
      });
  }

    #[test]
  fn test_handle_hello() {
    setup();
    let message = Message::Hello;
    let answer: Message  = handle_message(message);
    debug!("Answer: {:?}", answer);
    assert!(matches!(answer, Message::Welcome { version: 1 }));
  }

  #[test]
  fn test_handle_subscribe() {
    setup();
    let message = Message::Subscribe { name: "John".to_owned() };
    let answer: Message  = handle_message(message);
    println!("Answer: {:?}", answer);
    assert!(matches!(answer, Message::SubscribeResult(SubscribeResult::Ok)));
  }
}