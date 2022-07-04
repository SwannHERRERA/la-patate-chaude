use std::{sync::mpsc::Sender, net::{TcpStream, Shutdown}, io::{Read, Write}, time::Instant};

use hashcash::dto::{MD5HashCashInput, MD5HashCash};
use log::{trace, warn, info, debug};
use shared::{message::{Message, ResponseType, MessageType, PublicLeaderBoard}, challenge::ChallengeType};

use crate::{message_handler::MessageHandler, game::Game};

pub struct Exchanger {
  message_handler: MessageHandler,
  game: Game,
  tx: Sender<Message>,
}

impl Exchanger {
  pub fn new(message_handler: MessageHandler, tx: Sender<Message>, game: Game) -> Exchanger {
    Exchanger { message_handler, tx, game }
  }

  pub fn hold_communcation(&mut self, stream: TcpStream) {
    info!("peer address={:?}", stream.peer_addr());
    loop  {
      let parsed_message = self.parse_message_from_tcp_stream(&stream);
      if parsed_message.is_none() {
        continue;
      }
      let parsed_message = parsed_message.unwrap();
      match self.message_handler.handle_message(&parsed_message, &stream, self.message_handler.get_challenge()) {
        Some(response) => {
          if matches!(response.message, Message::EndOfCommunication) {
            self.game.players.set_player_inactive(&stream);
            break;
          }
          match response.message_type {
            ResponseType::Broadcast => {
              trace!("Broadcast: {:?}", response.message);
              let is_start_round = matches!(response.message, Message::PublicLeaderBoard(PublicLeaderBoard { .. }));
              self.tx.send(response.message).unwrap();
              if is_start_round {
                self.challenge();
              }
            }
            ResponseType::Unicast => {
              trace!("Unicast: {:?}", response.message);
              self.send_response(response.message, &stream);
            }
          }
        },
        None => {
          warn!("No response for message: {:?}", &parsed_message);
        }
      }
      let shutdown_result = stream.shutdown(Shutdown::Both);
      if shutdown_result.is_err() {
        trace!("Shutdown failed: {:?}", shutdown_result);
      }
    }
  }

  fn challenge(&mut self) {
    let mut now = self.game.round_timer.lock().unwrap();
    *now = Some(Instant::now());
    let challenge_message = self.start_round();
    let player_name = self.game.players.pick_random_player().unwrap().name;
    if let Some(mut player) = self.game.players.get_and_remove_player_by_name(&player_name) {
      player.send_message(challenge_message.message);
      self.game.players.add_player(player);
    }
  }

  fn parse_message_from_tcp_stream(&self, mut stream: &TcpStream) -> Option<Message> {
    let mut message_size = [0; 4];
    let _size_error = stream.read(&mut message_size);
    let decimal_size = u32::from_be_bytes(message_size);
    if decimal_size == 0 {
      return None;
    }
    debug!("decinmal size : {}", decimal_size);

    let mut bytes_of_message = vec![0; decimal_size as usize];
    debug!("bytes_of_message: {:?}", &bytes_of_message);
    let _size_read = stream.read_exact(&mut bytes_of_message);
    let message = String::from_utf8_lossy(&bytes_of_message);
    let message_clone = message.clone();
    let message = serde_json::from_str(&message);
    match message {
      Ok(m) => m,
      Err(err) => {
        debug!("Error parsing message: {:?}", &message_clone);
        warn!("Cannot parse message : {:?}", err);
        Some(Message::EndOfCommunication)
      },
    }
  }

  pub fn send_response(&self, response: Message, mut tcp_stream: &TcpStream) {
    let response = serde_json::to_string(&response);
    if response.is_err() {
      panic!("Cannot serialize message : {:?}", response)
    }
    let response = response.unwrap();
    let response = response.as_bytes();
    let response_size = response.len() as u32;
    let response_length_as_bytes = response_size.to_be_bytes();
    let result = tcp_stream.write(&[&response_length_as_bytes, response].concat());
    trace!("byte write : {:?}, ", result);
  }

  fn start_round(&self) -> MessageType {
    let challenge = ChallengeType::MD5HashCash(MD5HashCash(MD5HashCashInput::new()));

    let message = Message::Challenge(challenge);
    MessageType::unicast(message)
  }
}


