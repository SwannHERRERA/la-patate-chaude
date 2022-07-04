use game::Game;
use server::{create_listener, Server};
use shared::config;

mod player;
mod game;
mod message_handler;
mod exchanger;
mod server;
fn main() {
  std::env::set_var("RUST_LOG", config::LOG_LEVEL);
  pretty_env_logger::init();
  let listener = create_listener();
  let game = Game::new();
  let mut server: Server = Server::new(listener, game);
  server.listen();
}
