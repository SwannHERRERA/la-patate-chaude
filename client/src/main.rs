use std::{io::{Read, Write}, net::TcpStream, thread};
use std::collections::HashSet;
use std::net::Shutdown;
use std::sync::atomic::Ordering;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use clap::Parser;
use log::{debug, error, trace};
use rand;
use rand::Rng;

use hashcash::hashcash::{THREAD_COUNT, THREAD_SEED_SLICE};
use shared::challenge::{Challenge, ChallengeAnswer, ChallengeType, DictionaryChallenge};
use shared::config::LOG_LEVEL;
use shared::message::{Message, PublicLeaderBoard};
use shared::message::Message::ChallengeResult;
use shared::subscribe::SubscribeResult;
use utils::file_utils::read_file_macro;
use utils::string_utils::generate_dictionary_hashmap;

use crate::strategies::{
    BottomTargetStrategy, RandomTargetStrategy, TargetStrategy, TargetStrategyType,
    TopTargetStrategy,
};
use crate::ui::{ClientData, start_ui_display};

mod strategies;
mod ui;

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

    /// Use dictionary sentence for recover secret challenge
    #[clap(long, value_parser, default_value_t = false)]
    pub load_dictionary: bool,

    /// Enable cheat mode for recover secret challenge
    #[clap(long, value_parser, default_value_t = false)]
    pub cheat: bool,

    /// Enable client ui display
    #[clap(long, value_parser, default_value_t = false)]
    pub display_gui: bool,
}

fn main() {
    let args = ClientArgs::parse();
    THREAD_COUNT.store(args.thread_count, Ordering::Relaxed);
    THREAD_SEED_SLICE.store(args.thread_seed_slice, Ordering::Relaxed);
    std::env::set_var("RUST_LOG", LOG_LEVEL);
    pretty_env_logger::init();
    match TcpStream::connect(format!("{}:{}", args.ip, args.port).as_str()) {
        Ok(stream) => {
            let (ui_writer, ui_reader) = mpsc::channel();

            let client = Client::new(&args, ui_writer);
            if args.display_gui {
                start_ui_display(ui_reader);
            }
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
    dictionary_hashmap: &Option<HashSet<String>>,
    cheat: &bool,
) -> ChallengeAnswer {
    match challenge {
        ChallengeType::MD5HashCash(challenge) => ChallengeAnswer::MD5HashCash(challenge.solve()),
        ChallengeType::RecoverSecret(challenge) => {
            return if let Some(dictionary_hashmap) = dictionary_hashmap {
                if *cheat {
                    ChallengeAnswer::RecoverSecret(challenge.solve_secret_cheat())
                } else {
                    ChallengeAnswer::RecoverSecret(challenge.solve_secret(dictionary_hashmap))
                }
            } else {
                if *cheat {
                    ChallengeAnswer::RecoverSecret(challenge.solve_cheat())
                } else {
                    ChallengeAnswer::RecoverSecret(challenge.solve())
                }
            };
        }
        ChallengeType::MonstrousMaze(challenge) => {
            ChallengeAnswer::MonstrousMaze(challenge.solve())
        }
    }
}

pub struct Client {
    public_leader_board: PublicLeaderBoard,
    username: String,
    next_target_strategy: TargetStrategyType,
    dictionary_hashmap: Option<HashSet<String>>,
    cheat: bool,
    ui_enabled: bool,
    ui_writer: Sender<ClientData>,
}

impl Client {
    pub fn new(args: &ClientArgs, ui_writer: Sender<ClientData>) -> Client {
        let mut rng = rand::thread_rng();
        // Load dictionary file
        let dictionary_hashmap;
        let username = args.username.clone();
        if args.load_dictionary {
            debug!("Reading dictionary file...");
            let dictionary = read_file_macro();
            debug!("Generating hashmap...");
            dictionary_hashmap = Some(generate_dictionary_hashmap(&dictionary));
            debug!("Done !");
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
        ui_writer.send(ClientData { public_leader_board: vec![], username: username.clone() })
            .expect("Could not send public leader board message");
        Client {
            public_leader_board: vec![],
            username,
            next_target_strategy,
            dictionary_hashmap,
            cheat: args.cheat,
            ui_enabled: args.display_gui,
            ui_writer,
        }
    }

    pub fn start_threads(self, stream: TcpStream) {
        let (thread_writer, thread_reader) = mpsc::channel();

        let stream_cpy = stream.try_clone().expect("Could not clone stream");
        self.start_message_sender(stream, thread_reader);

        thread_writer.send(Message::Hello).expect("Could not send hello message");
        self.start_message_listener(stream_cpy, thread_writer);
    }

    fn start_message_listener(
        mut self,
        mut stream: TcpStream,
        thread_writer: Sender<Message>,
    ) {
        let mut buf_size = [0; 4];

        loop {
            stream.read(&mut buf_size).expect("Could not read stream message size");

            let res_size = u32::from_be_bytes(buf_size);
            if res_size == 0 {
                debug!("Received 0 size message");
                continue;
            }

            let mut buf = vec![0; res_size as usize];

            stream.read(&mut buf).expect("Could not read stream message");

            let string_receive = String::from_utf8_lossy(&buf);

            match serde_json::from_str(&string_receive) {
                Ok(message) => {
                    let message = self.dispatch_messages(message, &thread_writer);
                    match message {
                        Message::EndOfGame { .. } => {
                            debug!("Shutting down reader stream");
                            stream.shutdown(Shutdown::Both).expect("shutdown call failed");
                            break;
                        }
                        _ => {}
                    }
                },
                Err(err) => error!("Error while parsing message {:?}", err),
            }
        }
    }

    fn dispatch_messages(&mut self, message: Message, thread_writer: &Sender<Message>) -> Message {
        debug!("Dispatching: {:?}", message);
        match message.clone() {
            Message::Welcome { .. } => {
                let answer = Message::Subscribe {
                    name: self.username.clone(),
                };
                thread_writer.send(answer).expect("Could not send subscribe message");
            }
            Message::Challenge(challenge) => {
                let challenge_answer =
                    solve_challenge(challenge, &self.dictionary_hashmap, &self.cheat);

                let next_target = match self.next_target_strategy.clone() {
                    TargetStrategyType::RandomTargetStrategy(strategy) => {
                        strategy.next_target(&self.public_leader_board)
                    }
                    TargetStrategyType::TopTargetStrategy(strategy) => {
                        strategy.next_target(&self.public_leader_board)
                    }
                    TargetStrategyType::BottomTargetStrategy(strategy) => {
                        strategy.next_target(&self.public_leader_board)
                    }
                };
                debug!("Selected next target: {:?}", next_target);
                thread_writer
                    .send(ChallengeResult {
                        answer: challenge_answer,
                        next_target: next_target.to_string(),
                    })
                    .expect("Could not send challenge result message");
            }
            Message::PublicLeaderBoard(leader_board) => {
                self.public_leader_board = leader_board;
                if self.ui_enabled {
                    self.ui_writer.send(ClientData { public_leader_board: self.public_leader_board.clone(), username: self.username.clone() })
                        .expect("Could not send public leader board message");
                }
            }
            Message::SubscribeResult(result) => match result {
                SubscribeResult::Ok => {}
                SubscribeResult::Err(err) => {
                    panic!("{:?}", err);
                }
            }
            Message::RoundSummary { challenge: _, chain: _ } => {}
            Message::EndOfGame { leader_board } => {
                trace!("{:?}", leader_board);
                thread_writer
                    .send(Message::EndOfGame { leader_board })
                    .expect("Could not send end of game message");
            }
            _ => error!("Unhandled message {:?}", message),
        }
        message
    }

    fn start_message_sender(&self, mut stream: TcpStream, thread_reader: Receiver<Message>) {
        thread::spawn(move || {
            for message in thread_reader {
                match message {
                    Message::EndOfGame { .. } => {
                        debug!("Shutting down writer stream");
                        break;
                    }
                    _ => {
                        if let Ok(message) = serde_json::to_string(&message) {
                            let bytes_message = message.as_bytes();
                            let message_size = bytes_message.len() as u32;
                            let message_length_as_bytes = message_size.to_be_bytes();
                            let result =
                                stream.write(&[&message_length_as_bytes, bytes_message].concat());
                            debug!("Write result : {:?}, message: {}", result, message);
                        }else {
                            error!("Could not serialize message");
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
