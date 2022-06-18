use log::{info, debug, trace};
use shared::message::{Message, PublicLeaderBoard};
use shared::public_player::PublicPlayer;
use shared::subscribe::{SubscribeResult, SubscribeError};
#[derive(Debug)]
pub struct MessageHandler {
  players: Vec<PublicPlayer>,
}

impl MessageHandler {
  pub fn new(players: Vec<PublicPlayer>) -> MessageHandler {
    MessageHandler { players }
  }

  pub fn handle_message(&mut self, message: Message) -> Message {
      info!("Incomming Message: {:?}", message);
      match message {
        Message::Hello => self.handle_hello(),
        Message::Subscribe { name } => self.handle_subscribtion(name),
        Message::StartGame {  } => self.handle_start_game(),
        _ => panic!("Not implemented")
      }
  }

  fn handle_subscribtion(&mut self, name: String) -> Message {
    let answer = if self.has_player_with_name(&name) {
      Message::SubscribeResult(SubscribeResult::Err(SubscribeError::AlreadyRegistered))
    } else {
      Message::SubscribeResult(SubscribeResult::Ok)
    };
    let player = PublicPlayer::new(name, "".to_string(), 0, 0, true, 0.0);
    self.players.push(player);
    debug!("Answer: {:?}", answer);
    trace!("Players: {:?}", self.players);
    answer
  }

  fn handle_hello(&self) -> Message {
    let answer = Message::Welcome { version: 1 };
    debug!("Answer: {:?}", answer);
    answer
  }

  fn handle_start_game(&self) -> Message {
    let answer = Message::PublicLeaderBoard(self.players.clone());
    debug!("Answer: {:?}", answer);
    answer
  }

  fn has_player_with_name(&self, name: &str) -> bool {
    self.players.iter().any(|player| player.name == name)
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
    let mut handler = MessageHandler::new(vec![]);
    println!("{:?}", handler);
    let message = Message::Hello;
    let answer: Message  = handler.handle_message(message);
    debug!("Answer: {:?}", answer);
    assert!(matches!(answer, Message::Welcome { version: 1 }));
  }

  #[test]
  fn test_handle_subscribe() {
    setup();
    let mut handler = MessageHandler::new(vec![]);
    let message = Message::Subscribe { name: "John".to_owned() };
    let answer: Message  = handler.handle_message(message);
    println!("Answer: {:?}", answer);
    assert!(matches!(answer, Message::SubscribeResult(SubscribeResult::Ok)));
  }

  #[test]
  fn test_handle_subscribe_already_registered() {
    setup();
    let mut handler = MessageHandler::new(vec![PublicPlayer::new("John".to_owned(), "".to_owned(), 0, 0, true, 0.0)]);
    let answer = handler.handle_subscribtion("John".to_owned());
    assert!(matches!(answer, Message::SubscribeResult(SubscribeResult::Err(SubscribeError::AlreadyRegistered))));
  }
}