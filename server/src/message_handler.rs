use std::net::TcpStream;
use log::{info, debug, trace};
use shared::message::{Message, MessageType};
use shared::public_player::PublicPlayer;
use shared::subscribe::{SubscribeResult, SubscribeError};

use crate::player::{PlayerList, Player};
#[derive(Debug)]
pub struct MessageHandler {
  players: PlayerList,
}

impl MessageHandler {
  pub fn new(players: PlayerList) -> MessageHandler {
    MessageHandler { players }
  }

  pub fn handle_message(&mut self, message: Message, stream: &TcpStream) -> MessageType {
      info!("Incomming Message: {:?}", message);
      match message {
        Message::Hello => self.handle_hello(),
        Message::Subscribe { name } => self.handle_subscribtion(name, stream),
        Message::StartGame {  } => self.handle_start_game(),
        Message::EndOfCommunication =>self.handle_end_of_communication(stream),
        _ => panic!("Not implemented")
      }
  }

  fn handle_subscribtion(&mut self, name: String, stream: &TcpStream) -> MessageType {
    let answer = if self.players.has_player_with_name(&name) {
      Message::SubscribeResult(SubscribeResult::Err(SubscribeError::AlreadyRegistered))
    } else {
      Message::SubscribeResult(SubscribeResult::Ok)
    };
    let answer = MessageType::unicast(answer);
    let player = Player::new(PublicPlayer::new(name), stream.try_clone().unwrap());
    self.players.add_player(player);
    debug!("Answer: {:?}", answer);
    trace!("Players: {:?}", self.players);
    answer
  }

  fn handle_hello(&self) -> MessageType {
    let answer = MessageType::unicast(Message::Welcome { version: 1 });
    debug!("Answer: {:?}", answer);
    answer
  }

  fn handle_start_game(&self) -> MessageType {
    let strat_game_message = Message::PublicLeaderBoard(self.players.get_players());
    debug!("Start Game Message: {:?}", strat_game_message);
    let answer = MessageType::boardcast(strat_game_message);
    debug!("Answer: {:?}", answer);
    answer
  }

  fn handle_end_of_communication(&self, stream: &TcpStream) -> MessageType {
    let answer = MessageType::unicast(Message::EndOfCommunication);
    info!("stream id: {:?}", stream.peer_addr());
    debug!("Answer: {:?}", answer);
    answer
  }
}


// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::sync::Once;

//     static INIT: Once = Once::new();

//     pub fn setup() -> () {
//       INIT.call_once(|| {
//         std::env::set_var("RUST_LOG", "trace");
//       });
//   }

//     #[test]
//   fn test_handle_hello() {
//     setup();
//     let mut handler = MessageHandler::new(PlayerList::new());
//     println!("{:?}", handler);
//     let message = Message::Hello;
//     let answer = handler.handle_message(message);
//     debug!("Answer: {:?}", answer);
//     let _expected = MessageType::unicast(Message::Welcome { version: 1 });
//     assert!(matches!(answer, _expected));
//   }

//   #[test]
//   fn test_handle_subscribe() {
//     setup();
//     let mut handler = MessageHandler::new(PlayerList::new());
//     let message = Message::Subscribe { name: "John".to_owned() };
//     let answer = handler.handle_message(message);
//     debug!("Answer: {:?}", answer);
//     let _expected = MessageType::unicast(Message::SubscribeResult(SubscribeResult::Ok));
//     assert!(matches!(answer, _expected));
//   }

//   #[test]
//   fn test_handle_subscribe_already_registered() {
//     setup();
//     let mut handler = MessageHandler::new(PlayerList::new());
//     handler.players.add_player(Player::new(PublicPlayer::new("John".to_owned()), TcpStream::new()));
//     let answer = handler.handle_subscribtion("John".to_owned());
//     let _expected = MessageType::unicast(Message::SubscribeResult(SubscribeResult::Err(SubscribeError::AlreadyRegistered)));
//     assert!(matches!(answer, _expected));
//   }

//   #[test]
//   fn test_handle_start_game() {
//     setup();
//     let handler = MessageHandler::new(PlayerList::new());
//     handler.players.add_player(Player::new(PublicPlayer::new("John".to_owned()), TcpStream::new()));
//     let answer = handler.handle_start_game();
//     debug!("Answer: {:?}", answer);
//     let _expected = MessageType::boardcast(Message::PublicLeaderBoard(vec![PublicPlayer::new("John".to_owned())]));
//     assert!(matches!(answer, _expected));
//   }
// }