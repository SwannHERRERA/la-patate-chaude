use log::{debug, error, info, trace};
use shared::challenge::{
    get_name_of_challenge_type, Challenge, ChallengeAnswer, ChallengeType, ChallengeValue,
    ReportedChallengeResult,
};
use shared::message::{Message, MessageType};
use shared::subscribe::{SubscribeError, SubscribeResult};

use crate::game::Game;
#[derive(Debug)]
pub struct MessageHandler {
    game: Game,
}

impl MessageHandler {
    pub fn new(game: Game) -> MessageHandler {
        MessageHandler { game }
    }

    pub fn handle_message(
        &mut self,
        message: Message,
        client_id: String,
        current_challenge: Option<ChallengeType>,
    ) -> MessageType {
        debug!("Incoming Message: {:?}", message);
        match message {
            Message::Hello => self.handle_hello(client_id),
            Message::Subscribe { name } => self.handle_subscription(name, client_id),
            Message::StartGame {} => self.handle_start_game(),
            Message::ChallengeResult {
                answer,
                next_target,
            } => self.handle_challenge_result(current_challenge, answer, next_target, client_id),
            Message::EndOfCommunication => self.handle_end_of_communication(client_id),
            _ => panic!("Not implemented"),
        }
    }

    fn handle_subscription(&mut self, name: String, client_id: String) -> MessageType {
        let answer = if self.game.players.has_player_with_name(&name) {
            Message::SubscribeResult(SubscribeResult::Err(SubscribeError::AlreadyRegistered))
        } else {
            Message::SubscribeResult(SubscribeResult::Ok)
        };
        let answer = MessageType::unicast(answer, client_id.clone());
        self.game
            .players
            .activate_player(client_id.as_str(), name.as_str());
        trace!("Answer: {:?}", answer);
        trace!("game: {:?}", self.game);
        answer
    }

    fn handle_hello(&self, client_id: String) -> MessageType {
        let answer = MessageType::unicast(Message::Welcome { version: 1 }, client_id);
        trace!("Answer: {:?}", answer);
        answer
    }

    fn handle_start_game(&self) -> MessageType {
        let start_game_message = Message::PublicLeaderBoard(self.game.get_players());
        debug!("Start Game Message: {:?}", start_game_message);
        let answer = MessageType::boardcast(start_game_message);
        trace!("Answer: {:?}", answer);
        answer
    }

    fn handle_end_of_communication(&self, client_id: String) -> MessageType {
        info!("end of communication with client id: {:?}", client_id);
        let answer = MessageType::unicast(Message::EndOfCommunication, client_id);
        trace!("Answer: {:?}", answer);
        answer
    }

    fn handle_challenge_result(
        &mut self,
        challenge: Option<ChallengeType>,
        answer: ChallengeAnswer,
        next_target: String,
        client_id: String,
    ) -> MessageType {
        match challenge {
            Some(challenge) => {
                if self.has_pass_challenge(answer, &challenge) {
                    self.game.update_winner(client_id.as_str());
                }
                let challenge_result = ReportedChallengeResult {
                    name: get_name_of_challenge_type(&self.game.game_type),
                    value: ChallengeValue::Ok {
                        used_time: 0.0,
                        next_target,
                    },
                };
                self.game.push_reported_challenge_result(challenge_result);
                trace!("get chain: {:?}", self.game.get_chain());
                MessageType::boardcast(Message::RoundSummary {
                    challenge: get_name_of_challenge_type(&self.game.game_type),
                    chain: self.game.get_chain(),
                })
            }
            None => {
                error!("No challenge to answer");
                panic!("No challenge to answer, current_challenge is None");
            }
        }
    }
    fn has_pass_challenge(&self, answer: ChallengeAnswer, challenge: &ChallengeType) -> bool {
        match answer {
            ChallengeAnswer::MD5HashCash(output) => match challenge {
                ChallengeType::MD5HashCash(challenge) => challenge.verify(output),
                _ => panic!("Challenge is not MD5HashCash"),
            },
            ChallengeAnswer::RecoverSecret(output) => match challenge {
                ChallengeType::RecoverSecret(challenge) => challenge.verify(output),
                _ => panic!("Challenge is not RecoverSecret"),
            },
            ChallengeAnswer::MonstrousMaze(output) => match challenge {
                ChallengeType::MonstrousMaze(challenge) => challenge.verify(output),
                _ => panic!("Challenge is not MonstrousMaze"),
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
