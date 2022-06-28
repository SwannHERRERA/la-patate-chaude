use std::{io::{Read, Write}, net::TcpStream, thread};
use std::net::{Shutdown, SocketAddr};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;

use log::{debug, error, trace, warn};
use rand;
use rand::Rng;

use shared::challenge::{Challenge, ChallengeAnswer, ChallengeType};
use shared::config;
use shared::config::{IP, PORT};
use shared::message::{Message, PublicLeaderBoard};
use shared::message::Message::ChallengeResult;
use shared::subscribe::SubscribeResult;

fn main() {
    std::env::set_var("RUST_LOG", config::LOG_LEVEL);
    let address = SocketAddr::from((IP, PORT));
    match TcpStream::connect(address) {
        Ok(stream) => {
            let client = Client::new();
            client.start_threads(stream);
        },
        Err(_) => panic!("Could not connect to server {:?} on port {}", IP, PORT),
    }
}

fn solve_challenge(challenge: ChallengeType) -> ChallengeAnswer {
    match challenge {
        ChallengeType::MD5HashCash(challenge) => {
            ChallengeAnswer::MD5HashCash(challenge.solve())
        }
    }
}

pub struct Client {
    public_leader_board: PublicLeaderBoard,
    username: String,
}

impl Client {
    fn new() -> Client {
        let mut rng = rand::thread_rng();
        let n1: u8 = rng.gen();
        let username = "test".to_string() + &*n1.to_string();
        Client {
            public_leader_board: vec![],
            username,
        }
    }

    fn start_threads(self, stream: TcpStream) {
        let (thread_writer, thread_reader) = mpsc::channel();
        let stream_cpy = stream.try_clone().unwrap();
        self.start_message_sender(stream, thread_reader);
        thread_writer.send(Message::Hello).unwrap();
        self.start_message_listener(stream_cpy, thread_writer.clone());
    }

    fn start_message_listener(mut self, mut stream: TcpStream, thread_writer: Sender<Message>) -> JoinHandle<()> {
        loop {
            let mut buf_size = [0; 4];
            stream.read(&mut buf_size).unwrap();
            let res_size = u32::from_be_bytes(buf_size);
            if res_size == 0 {
                continue
            }

            let mut buf = vec![0; res_size as usize];
            stream.read(&mut buf).unwrap();
            let string_receive = String::from_utf8_lossy(&buf);

            match serde_json::from_str(&string_receive) {
                Ok(message) => self.dispatch_messages(message, &thread_writer),
                Err(err) => error!("Error while parsing message {:?}", err),
            }
        }
    }

    fn dispatch_messages(&mut self, message: Message, thread_writer: &Sender<Message>) {
        debug!("Dispatching: {:?}", message);
        match message {
            Message::Welcome { .. } => {
                let answer = Message::Subscribe { name: self.username.clone() };
                thread_writer.send(answer).unwrap();
            }
            Message::Challenge(challenge) => {
                let challenge_answer = solve_challenge(challenge);
                let next_target = select_next_user(self.username.clone(), &mut self.public_leader_board).clone();
                debug!("Selected next target: {:?}",next_target);
                thread_writer.send(ChallengeResult { answer: challenge_answer, next_target: next_target.to_string() }).unwrap();
            }
            Message::PublicLeaderBoard(leader_board) => {
                self.public_leader_board = leader_board;
            }
            Message::SubscribeResult(result) => {
                match result {
                    SubscribeResult::Ok => {}
                    SubscribeResult::Err(err) => {
                        panic!("{:?}", err);
                    }
                }
            }
            Message::RoundSummary { challenge, chain } => {}
            Message::EndOfGame { leader_board } => {
                trace!("{:?}", leader_board);
                thread_writer.send(Message::EndOfGame { leader_board }).unwrap();
            }
            _ => warn!("Unhandled message {:?}", message)
        }
    }

    fn start_message_sender(&self, mut stream: TcpStream, thread_reader: Receiver<Message>) {
        thread::spawn(move || {
            for message in thread_reader {
                match message {
                    Message::EndOfGame { .. } => {
                        debug!("Shutting down stream");
                        stream.shutdown(Shutdown::Both).expect("shutdown call failed");
                    }
                    _ => {
                        if let Ok(message) = serde_json::to_string(&message) {
                            let bytes_message = message.as_bytes();
                            let message_size = bytes_message.len() as u32;
                            let message_length_as_bytes = message_size.to_be_bytes();
                            let result = stream.write(&[&message_length_as_bytes, bytes_message].concat());
                            debug!("Write result : {:?}, message: {}", result, message);
                        }
                    }
                }
            }
        });
    }
}

fn select_next_user(username: String, public_leader_board: &mut PublicLeaderBoard) -> String {
    public_leader_board.sort_by(|a, b| b.score.cmp(&a.score));
    for player in public_leader_board {
        if player.name != username && player.is_active {
            return player.name.clone()
        }
    }
    "".to_string()
}
