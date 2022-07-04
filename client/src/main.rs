use std::collections::HashMap;
use std::net::{Shutdown, SocketAddr};
use std::sync::atomic::Ordering;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;
use std::{
    io::{Read, Write},
    net::TcpStream,
    thread,
};

use clap::Parser;
use log::{debug, error, trace, warn};
use rand;
use rand::Rng;

use hashcash::hashcash::{THREAD_COUNT, THREAD_SEED_SLICE};
use shared::challenge::{Challenge, ChallengeAnswer, ChallengeType, DictionaryChallenge};
use shared::config::{IP, LOG_LEVEL, PORT};
use shared::message::Message::ChallengeResult;
use shared::message::{Message, PublicLeaderBoard};
use shared::subscribe::SubscribeResult;
use utils::file_utils::read_file_macro;
use utils::string_utils::generate_dictionary_hashmap;

use crate::strategies::{
    BottomTargetStrategy, RandomTargetStrategy, TargetStrategy, TargetStrategyType,
    TopTargetStrategy,
};

mod strategies;

/// Client configuration
#[derive(Parser, Default, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct ClientArgs {
    /// Name of the player must be unique
    #[clap(short, long, value_parser, default_value = generate_random_username())]
    username: String,

    /// Server IP
    #[clap(short, value_parser, default_value = "127.0.0.1")]
    ip: String,

    /// Server port
    #[clap(long, value_parser, default_value_t = 7878)]
    port: u16,

    /// Threads for challenge solving
    #[clap(long, value_parser, default_value_t = num_cpus::get())]
    thread_count: usize,

    /// The number of seed incrementation
    #[clap(long, value_parser, default_value_t = 1000)]
    thread_seed_slice: u64,

    /// If use dictionary for recover secret challenge
    #[clap(long, value_parser, default_value_t = false)]
    pub load_dictionary: bool,
}

fn main() {
    let args = ClientArgs::parse();
    THREAD_COUNT.store(args.thread_count, Ordering::Relaxed);
    THREAD_SEED_SLICE.store(args.thread_seed_slice, Ordering::Relaxed);
    std::env::set_var("RUST_LOG", LOG_LEVEL);
    let address = SocketAddr::from((IP, PORT));

    match TcpStream::connect(address) {
        Ok(stream) => {
            let client = Client::new(args.username, args.load_dictionary);
            client.start_threads(stream);
        }
        Err(_) => panic!(
            "Could not connect to server {:?} on port {}",
            args.ip, args.port
        ),
    }
}

fn solve_challenge(
    challenge: ChallengeType,
    dictionary_hashmap: &Option<HashMap<char, Vec<String>>>,
) -> ChallengeAnswer {
    match challenge {
        ChallengeType::MD5HashCash(challenge) => ChallengeAnswer::MD5HashCash(challenge.solve()),
        ChallengeType::RecoverSecret(challenge) => {
            if let Some(dictionary_hashmap) = dictionary_hashmap {
                ChallengeAnswer::RecoverSecret(challenge.solve_secret(dictionary_hashmap))
            } else {
                ChallengeAnswer::RecoverSecret(challenge.solve())
            }
        }
    }
}

pub struct Client {
    public_leader_board: PublicLeaderBoard,
    username: String,
    next_target_strategy: TargetStrategyType,
    dictionary_hashmap: Option<HashMap<char, Vec<String>>>,
}

impl Client {
    fn new(username: String, load_dictionary: bool) -> Client {
        let mut rng = rand::thread_rng();
        // Load dictionary file
        let dictionary_hashmap;
        if load_dictionary {
            println!("Reading dictionary file...");
            let dictionary = read_file_macro();
            println!("Generating hashmap...");
            dictionary_hashmap = Some(generate_dictionary_hashmap(&dictionary));
            println!("Done !");
        } else {
            dictionary_hashmap = None;
        }

        let next_target_strategy = match rng.gen_range(0..=2) {
            0 => TargetStrategyType::TopTargetStrategy(TopTargetStrategy {
                current_name: username.clone(),
            }),
            1 => TargetStrategyType::BottomTargetStrategy(BottomTargetStrategy {
                current_name: username.clone(),
            }),
            2 => TargetStrategyType::RandomTargetStrategy(RandomTargetStrategy {
                current_name: username.clone(),
            }),
            _ => {
                panic!("Cannot find strategy type")
            }
        };
        debug!("Selected strategy : {:?}", next_target_strategy);
        Client {
            public_leader_board: vec![],
            username,
            next_target_strategy,
            dictionary_hashmap,
        }
    }

    fn start_threads(self, stream: TcpStream) {
        let (thread_writer, thread_reader) = mpsc::channel();
        let stream_cpy = stream.try_clone().unwrap();
        self.start_message_sender(stream, thread_reader);
        thread_writer.send(Message::Hello).unwrap();
        self.start_message_listener(stream_cpy, thread_writer.clone());
    }

    fn start_message_listener(
        mut self,
        mut stream: TcpStream,
        thread_writer: Sender<Message>,
    ) -> JoinHandle<()> {
        loop {
            let mut buf_size = [0; 4];
            stream.read(&mut buf_size).unwrap();
            let res_size = u32::from_be_bytes(buf_size);
            if res_size == 0 {
                continue;
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
                let answer = Message::Subscribe {
                    name: self.username.clone(),
                };
                thread_writer.send(answer).unwrap();
            }
            Message::Challenge(challenge) => {
                let challenge_answer = solve_challenge(challenge, &self.dictionary_hashmap);
                let next_target = match self.next_target_strategy.clone() {
                    TargetStrategyType::RandomTargetStrategy(strategy) => {
                        strategy.next_target(self.public_leader_board.clone())
                    }
                    TargetStrategyType::TopTargetStrategy(strategy) => {
                        strategy.next_target(self.public_leader_board.clone())
                    }
                    TargetStrategyType::BottomTargetStrategy(strategy) => {
                        strategy.next_target(self.public_leader_board.clone())
                    }
                };
                debug!("Selected next target: {:?}", next_target);
                thread_writer
                    .send(ChallengeResult {
                        answer: challenge_answer,
                        next_target: next_target.to_string(),
                    })
                    .unwrap();
            }
            Message::PublicLeaderBoard(leader_board) => {
                self.public_leader_board = leader_board;
            }
            Message::SubscribeResult(result) => match result {
                SubscribeResult::Ok => {}
                SubscribeResult::Err(err) => {
                    panic!("{:?}", err);
                }
            },
            Message::RoundSummary {
                challenge: _,
                chain: _,
            } => {}
            Message::EndOfGame { leader_board } => {
                trace!("{:?}", leader_board);
                thread_writer
                    .send(Message::EndOfGame { leader_board })
                    .unwrap();
            }
            _ => warn!("Unhandled message {:?}", message),
        }
    }

    fn start_message_sender(&self, mut stream: TcpStream, thread_reader: Receiver<Message>) {
        thread::spawn(move || {
            for message in thread_reader {
                match message {
                    Message::EndOfGame { .. } => {
                        debug!("Shutting down stream");
                        stream
                            .shutdown(Shutdown::Both)
                            .expect("shutdown call failed");
                    }
                    _ => {
                        if let Ok(message) = serde_json::to_string(&message) {
                            let bytes_message = message.as_bytes();
                            let message_size = bytes_message.len() as u32;
                            let message_length_as_bytes = message_size.to_be_bytes();
                            let result =
                                stream.write(&[&message_length_as_bytes, bytes_message].concat());
                            debug!("Write result : {:?}, message: {}", result, message);
                        }
                    }
                }
            }
        });
    }
}

fn generate_random_username() -> &'static str {
    let mut rng = rand::thread_rng();
    let n1: u8 = rng.gen();
    let username = "user".to_string() + &*n1.to_string();
    Box::leak(username.into_boxed_str())
}
