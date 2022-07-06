use std::{sync::mpsc::Sender, net::{TcpStream, Shutdown}, io::Read};

use hashcash::dto::{MD5HashCashInput, MD5HashCash};
use log::{trace, warn, info, error};
use shared::{message::{Message, MessageType, PublicLeaderBoard}, challenge::ChallengeType};

use crate::{game::Game, message_handler::MessageHandler};

pub struct Exchanger {
  message_handler: MessageHandler,
  game: Game,
  tx: Sender<MessageType>,
}

impl Exchanger {

  pub fn new(message_handler: MessageHandler, game: Game, tx: Sender<MessageType>) -> Exchanger {
    Exchanger { message_handler, game, tx }
  }

  pub fn hold_communcation(&mut self, stream: TcpStream) {
    let client_id = stream.peer_addr().unwrap().to_string();
    info!("peer address={:?}", &client_id);
    loop {
      let parsed_message = self.parse_message_from_tcp_stream(&stream);
      let response = self.message_handler.handle_message(parsed_message, client_id.clone(), self.game.get_challenge());
      if matches!(response.message, Message::EndOfCommunication) {
        break;
      }
      let is_start_round = matches!(response.message, Message::PublicLeaderBoard(PublicLeaderBoard { .. }));
      self.tx.send(response).unwrap();
      if is_start_round {
        let challenge_message = self.start_round();
        self.tx.send(challenge_message).unwrap();
      }
    }
    let shutdown_result = stream.shutdown(Shutdown::Both);
    if shutdown_result.is_err() {
      trace!("Shutdown failed: {:?}", shutdown_result);
    }
  }

  fn parse_message_from_tcp_stream(&self, mut stream: &TcpStream) -> Message {
    let mut message_size = [0; 4];
    let _size_error = stream.read(&mut message_size);
    let decimal_size = u32::from_be_bytes(message_size);

    let mut bytes_of_message = vec![0; decimal_size as usize];
    let _size_read = stream.read_exact(&mut bytes_of_message);
    let message = String::from_utf8_lossy(&bytes_of_message);
    let message = serde_json::from_str(&message);
    match message {
      Ok(m) => m,
      Err(err) => {
        warn!("Cannot parse message : {:?}", err);
        Message::EndOfCommunication
      },
    }
  }

  fn start_round(&self) -> MessageType {
    let challenge = match self.game.challenge_type.as_str() {
      "hashcash" => ChallengeType::MD5HashCash(MD5HashCash(MD5HashCashInput::new())),
      _ => panic!("Unknown challenge type"),
    };
    self.game.set_challenge(challenge.clone());

    let message = Message::Challenge(challenge);
    let player = self.game.players.pick_random_player();
    if player.is_none() {
      error!("No player found");
      panic!("No player found");
    }
    let player = player.unwrap();
    MessageType::unicast(message, player.name)
  }
}


