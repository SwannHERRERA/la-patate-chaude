use std::{io::{Read, Write}, net::TcpStream, thread};
use std::net::SocketAddr;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;
use std::{
    io::{Read, Write},
    net::TcpStream,
    thread,
};

use rand;
use rand::Rng;

use shared::challenge::Challenge;
use shared::config::{IP, PORT};
use shared::message::Message::ChallengeResult;
use shared::message::{ChallengeAnswer, ChallengeType, Message, PublicLeaderBoard};

fn main() {
    let address = SocketAddr::from((IP, PORT));
    match TcpStream::connect(address) {
        Ok(stream) => {
            let client = Client::new();
            client.start_threads(stream);
        }
        Err(_) => panic!("Could not connect to server {:?} on port {}", IP, PORT),
    }
}

fn solve_challenge(challenge: ChallengeType) -> ChallengeAnswer {
    match challenge {
        ChallengeType::MD5HashCash(challenge) => ChallengeAnswer::MD5HashCash(challenge.solve()),
        ChallengeType::RecoverSecret(challenge) => {
            ChallengeAnswer::RecoverSecret(challenge.solve())
        }
    }
}

pub struct Client {
    public_leader_board: PublicLeaderBoard,
}

impl Client {
    fn new() -> Client {
        Client {
            public_leader_board: vec![],
        }
    }

    fn start_threads(self, stream: TcpStream) {
        let (thread_writer, thread_reader) = mpsc::channel();
        let stream_cpy = stream.try_clone().unwrap();
        self.start_message_sender(stream, thread_reader);
        thread_writer.send(Message::Hello).unwrap();
        self.start_message_listener(stream_cpy, thread_writer.clone());
    }

    fn start_message_listener(self, mut stream: TcpStream, thread_writer: Sender<Message>) -> JoinHandle<()> {
        loop {
            let mut buf_size = [0; 4];
            stream.read(&mut buf_size);
            let res_size = u32::from_be_bytes(buf_size);
            if res_size == 0 {
                continue;
            }

            let mut buf = vec![0; res_size as usize];
            stream.read(&mut buf);
            let string_receive = String::from_utf8_lossy(&buf);

            match serde_json::from_str(&string_receive) {
                Ok(message) => self.dispatch_messages(message, &thread_writer),
                Err(_) => println!("Error while parsing message"),
            }
        }
    }

    fn dispatch_messages(&self, message: Message, thread_writer: &Sender<Message>) {
        println!("Dispatching: {:?}", message);
        match message {
            Message::Welcome { .. } => {
                let mut rng = rand::thread_rng();
                let n1: u8 = rng.gen();
                let answer = Message::Subscribe { name: "test".to_string() + &*n1.to_string() };
                thread_writer.send(answer).unwrap();
            }
            Message::Challenge(challenge) => {
                let challenge_answer = solve_challenge(challenge);
                thread_writer.send(ChallengeResult { answer: challenge_answer, next_target: "".to_string() }).unwrap();
            }
            _ => {}
        }
    }

    fn start_message_sender(&self, mut stream: TcpStream, thread_reader: Receiver<Message>) {
        thread::spawn(move || {
            for message in thread_reader {
                if let Ok(message) = serde_json::to_string(&message) {
                    println!("Writing {:?}", message);
                    let bytes_message = message.as_bytes();
                    let message_size = bytes_message.len() as u32;
                    let message_length_as_bytes = message_size.to_be_bytes();
                    let result = stream.write(&[&message_length_as_bytes, bytes_message].concat());
                    println!("Write result : {:?}, message: {}", result, message);
                }
            }
        });
    }
}
