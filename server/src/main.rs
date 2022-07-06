use server::{create_listener, Server};
use shared::config;

mod utils;
mod player;
mod message_handler;
mod exchanger;
mod server;
mod game;

fn main() {
  std::env::set_var("RUST_LOG", config::LOG_LEVEL);
  pretty_env_logger::init();
  let listener = create_listener();
  let mut server: Server = Server::new(listener, config::CHALLENGE_TYPE);
  server.listen();
}
