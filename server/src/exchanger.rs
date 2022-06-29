use std::{sync::mpsc::Sender, net::{TcpStream, Shutdown}, io::{Read, Write}};

use hashcash::dto::{MD5HashCashInput, MD5HashCash};
use log::{trace, warn, info};
use shared::{message::{Message, ResponseType, MessageType, PublicLeaderBoard}, challenge::ChallengeType};

use crate::{message_handler::MessageHandler, player::PlayerList};

pub struct Exchanger {
  message_handler: MessageHandler,
  players: PlayerList,
  tx: Sender<Message>,
}

impl Exchanger {

  pub fn new(message_handler: MessageHandler, tx: Sender<Message>, players: PlayerList) -> Exchanger {
    Exchanger { message_handler, tx, players }
  }

  pub fn hold_communcation(&mut self, stream: TcpStream) {
    info!("peer address={:?}", stream.peer_addr());
    loop  {
      let parsed_message = self.parse_message_from_tcp_stream(&stream);
      if let Some(response) = self.message_handler.handle_message(parsed_message, &stream, self.message_handler.get_challenge()) {
        if matches!(response.message, Message::EndOfCommunication) {
          break;
        }
        match response.message_type {
          ResponseType::Broadcast => {
            trace!("Broadcast: {:?}", response.message);
            let is_start_round = matches!(response.message, Message::PublicLeaderBoard(PublicLeaderBoard { .. }));
            self.tx.send(response.message).unwrap();
            if is_start_round {
              let challenge_message = self.start_round();
              self.tx.send(challenge_message.message).unwrap();
            }
          }
          ResponseType::Unicast => {
            trace!("Unicast: {:?}", response.message);
            self.send_response(response.message, &stream);
          }
        }
      }
      let shutdown_result = stream.shutdown(Shutdown::Both);
      if shutdown_result.is_err() {
        trace!("Shutdown failed: {:?}", shutdown_result);
      }
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

  pub fn send_response(&self, response: Message, mut tcp_stream: &TcpStream) {
    let response = serde_json::to_string(&response).unwrap();
    let response = response.as_bytes();
    let response_size = response.len() as u32;
    let response_length_as_bytes = response_size.to_be_bytes();
    let result = tcp_stream.write(&[&response_length_as_bytes, response].concat());
    trace!("byte write : {:?}, ", result);
  }

  fn start_round(&self, next_target: String) -> MessageType {
    let challenge = ChallengeType::MD5HashCash(MD5HashCash(MD5HashCashInput::new()));

    let message = Message::Challenge(challenge);
    MessageType::boardcast(message)
  }
}


