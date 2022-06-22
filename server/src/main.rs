use message_handler::MessageHandler;
use server::{Server, create_listener};
use shared::config;

mod player;
mod message_handler;
mod exchanger;
mod server;

fn main() {
  std::env::set_var("RUST_LOG", config::LOG_LEVEL);
  pretty_env_logger::init();
  let listener = create_listener();
  let playerList = player::PlayerList::new();
  let message_handler = MessageHandler::new(playerList.clone());
  let mut server = Server::new(listener, message_handler, playerList.clone());
  server.listen();
}

