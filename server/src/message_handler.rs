use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use hashcash::dto::{MD5HashCash, MD5HashCashOutput};
use log::{info, debug, trace, error};
use shared::challenge::{ChallengeType, ChallengeAnswer, Challenge};
use shared::message::{Message, MessageType};
use shared::public_player::PublicPlayer;
use shared::subscribe::{SubscribeResult, SubscribeError};

use crate::game::Game;
use crate::player::{PlayerList, Player};
#[derive(Debug)]
pub struct MessageHandler {
  players: PlayerList,
  challenge: Arc<Mutex<Option<ChallengeType>>>,
}

impl MessageHandler {
  pub fn new(players: PlayerList, challenge: Arc<Mutex<Option<ChallengeType>>>) -> MessageHandler {
    MessageHandler { players, challenge }
  }

  pub fn new_from_game(game: &Game) -> MessageHandler {
    let players = game.players.clone();
    let challenge = game.current_chanllenge.clone();
    MessageHandler::new(players, challenge)
  }

  pub fn get_challenge(&self) -> Option<ChallengeType> {
    self.challenge.lock().unwrap().clone()
  }

  pub fn handle_message(&mut self, message: Message, stream: &TcpStream, current_challenge: Option<ChallengeType>) -> Option<MessageType> {
      info!("Incomming Message: {:?}", message);
      match message {
        Message::Hello => self.handle_hello(),
        Message::Subscribe { name } => self.handle_subscribtion(name, stream),
        Message::StartGame {  } => self.handle_start_game(),
        Message::ChallengeResult { answer, next_target } => self.handle_challenge_result(answer, next_target, current_challenge),
        Message::EndOfCommunication =>self.handle_end_of_communication(stream),
        _ => panic!("Not implemented")
      }
  }

  fn handle_subscribtion(&mut self, name: String, stream: &TcpStream) -> Option<MessageType> {
    let answer = if self.players.has_player_with_name(&name) {
      Message::SubscribeResult(SubscribeResult::Err(SubscribeError::AlreadyRegistered))
    } else {
      Message::SubscribeResult(SubscribeResult::Ok)
    };
    let answer = MessageType::unicast(answer);
    let stream_id =  stream.peer_addr().unwrap().to_string();
    let player = Player::new(PublicPlayer::new(name, stream_id), stream.try_clone().unwrap());
    self.players.add_player(player);
    debug!("Answer: {:?}", answer);
    trace!("Players: {:?}", self.players);
    Some(answer)
  }

  fn handle_hello(&self) -> Option<MessageType> {
    let answer = MessageType::unicast(Message::Welcome { version: 1 });
    debug!("Answer: {:?}", answer);
    Some(answer)
  }

  fn handle_start_game(&self) -> Option<MessageType> {
    if self.players.len() == 0 {
      return None;
    }
    let start_game_message = Message::PublicLeaderBoard(self.players.get_players());
    debug!("Start Game Message: {:?}", start_game_message);
    let answer = MessageType::boardcast(start_game_message);
    debug!("Answer: {:?}", answer);
    Some(answer)
  }

  fn handle_end_of_communication(&self, stream: &TcpStream) -> Option<MessageType> {
    let answer = MessageType::unicast(Message::EndOfCommunication);
    info!("stream id: {:?}", stream.peer_addr());
    debug!("Answer: {:?}", answer);
    Some(answer)
  }

  fn handle_challenge_result(&self, answer: ChallengeAnswer, _next_target: String, challenge: Option<ChallengeType>) -> Option<MessageType> {
    match challenge {
      Some(challenge) => {
        let (challenge, answer) = self.handle_md5(challenge, answer);
        if challenge.verify(answer) {
          // increase score of winning player
          return Some(MessageType::boardcast(Message::PublicLeaderBoard(self.players.get_players())));
        }
        None
      }
      None => {
        error!("No challenge to answer");
        panic!("No challenge to answer, current_challenge is None");
      }
    }
  }

  fn handle_md5(&self, challenge: ChallengeType, answer: ChallengeAnswer) -> (MD5HashCash, MD5HashCashOutput) {
    match challenge {
      ChallengeType::MD5HashCash(challenge) => {
      match answer {
        ChallengeAnswer::MD5HashCash(answer) => {
          return (challenge, answer);
        },
      }
      },
  } 
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