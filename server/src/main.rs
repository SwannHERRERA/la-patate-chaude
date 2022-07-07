use server::{create_listener, Server};
use shared::config;

mod utils;
mod player;
mod game;
mod message_handler;
mod exchanger;
mod server;


fn main() {
  std::env::set_var("RUST_LOG", config::LOG_LEVEL);
  pretty_env_logger::init();
  let listener = create_listener();
  let game = game::Game::new(config::GAME_TYPE);
  let mut server: Server = Server::new(listener, game);
  server.listen();
}
