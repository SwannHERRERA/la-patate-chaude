use std::{io::Write, net::TcpStream};

use log::trace;
use shared::message::Message;

pub fn send_response(response: Message, mut tcp_stream: &TcpStream) {
    let response = serde_json::to_string(&response).unwrap();
    let response = response.as_bytes();
    let response_size = response.len() as u32;
    let response_length_as_bytes = response_size.to_be_bytes();
    let result = tcp_stream.write(&[&response_length_as_bytes, response].concat());
    trace!("byte write : {:?}, ", result);
}
