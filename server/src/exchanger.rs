use std::{
    io::Read,
    net::{Shutdown, TcpStream},
    sync::mpsc::Sender,
};

use log::{debug, error, info, trace, warn};

use hashcash::dto::{MD5HashCash, MD5HashCashInput};
use monstrous_maze::{
    challenge_generator::generate_monstrous_maze_challenge, models::MonstrousMaze,
};
use recover_secret::{challenge_generator::generate_challenge, models::RecoverSecret};
use shared::{
    challenge::{ChallengeType, ChallengeValue, GameType},
    message::{Message, MessageType, PublicLeaderBoard},
};

use crate::{game::Game, message_handler::MessageHandler};

pub struct Exchanger {
    message_handler: MessageHandler,
    game: Game,
    tx: Sender<MessageType>,
}

impl Exchanger {
    pub fn new(message_handler: MessageHandler, game: Game, tx: Sender<MessageType>) -> Exchanger {
        Exchanger {
            message_handler,
            game,
            tx,
        }
    }

    pub fn hold_communication(&mut self, stream: TcpStream) {
        let client_id = stream
            .peer_addr()
            .expect("Cannot retrieve client address")
            .to_string();
        info!("peer address={:?}", &client_id);
        loop {
            let parsed_message = self.parse_message_from_tcp_stream(&stream);
            let response = self.message_handler.handle_message(
                parsed_message,
                client_id.clone(),
                self.game.get_challenge(),
            );

            if matches!(response.message, Message::EndOfCommunication) {
                break;
            }
            self.check_start_round(response.clone());
            self.check_end_challenge(response, client_id.clone());
        }

        let shutdown_result = stream.shutdown(Shutdown::Both);
        if shutdown_result.is_err() {
            trace!("Shutdown failed: {:?}", shutdown_result);
        }
    }

    fn check_end_challenge(&mut self, response: MessageType, client_id: String) {
        let mut is_end_of_round = false;
        if matches!(response.message, Message::RoundSummary { .. }) {
            let mut current_round = self.game.current_round.lock().unwrap();
            if let Some(current_round) = &mut *current_round {
                if current_round.start.elapsed() > current_round.duration {
                    let actual_player = current_round
                        .actual_player
                        .clone()
                        .expect("No actual player when challenge end");
                    self.game.update_score(actual_player.as_str());
                    info!("current round: {:?}", current_round);
                    is_end_of_round = true;
                }
            }
            drop(current_round);
            if is_end_of_round {
                self.game.push_current_round();
                let message = self.start_round();
                self.tx
                    .send(message)
                    .expect("Cannot send message, no receiver.");
                return;
            }

            trace!("End of challenge");
            let challenge = self.get_new_challenge();
            trace!("chain: {:?}", self.game.chain);
            if let Some(challenge_result) = self.game.get_last_chain_result() {
                debug!("{:?}", challenge_result);
                match &challenge_result.value {
                    ChallengeValue::Unreachable | ChallengeValue::Timeout => {
                        self.game.players.disable_player(client_id)
                    }
                    ChallengeValue::BadResult {
                        used_time: _,
                        next_target,
                    }
                    | ChallengeValue::Ok {
                        used_time: _,
                        next_target,
                    } => {
                        let message = Message::Challenge(challenge);
                        if let Some(player) = self.game.get_player_by_name(next_target) {
                            self.game.set_active_player(player.name.clone());
                            self.tx
                                .send(MessageType::unicast(message, player.stream_id))
                                .expect("Cannot send message, no receiver.");
                        }
                    }
                }
            }
        }
    }

    fn check_start_round(&mut self, response: MessageType) {
        let is_start_round = matches!(
            response.message,
            Message::PublicLeaderBoard(PublicLeaderBoard { .. })
        );
        self.tx
            .send(response)
            .expect("Cannot send message, no receiver");
        if is_start_round {
            let challenge_message = self.start_round();
            self.tx
                .send(challenge_message)
                .expect("Cannot send message, no receiver");
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
            }
        }
    }

    fn start_round(&self) -> MessageType {
        trace!("start round");
        let challenge = self.get_new_challenge();
        self.game.set_challenge(challenge.clone());

        let message = Message::Challenge(challenge);
        let player = self.game.players.pick_random_active_player();
        if player.is_none() {
            error!("No player found");
            panic!("No player found");
        }
        let player = player.unwrap();
        self.game.start_round();
        self.game.set_active_player(player.name.clone());

        MessageType::unicast(message, player.stream_id)
    }

    fn get_new_challenge(&self) -> ChallengeType {
        match self.game.game_type {
            GameType::HashCash => ChallengeType::MD5HashCash(MD5HashCash(MD5HashCashInput::new())),
            GameType::RecoverSecret => {
                ChallengeType::RecoverSecret(RecoverSecret(generate_challenge()))
            }

            GameType::MonstrousMaze => {
                ChallengeType::MonstrousMaze(MonstrousMaze(generate_monstrous_maze_challenge()))
            }
        }
    }
}
