use std::time::Duration;

use args::ServerArgs;
use clap::Parser;
use game::Game;
use server::{create_listener, Server};
use shared::challenge::GameType;

mod args;
mod utils;
mod player;
mod game;
mod message_handler;
mod exchanger;
mod server;


fn main() {
  let args = ServerArgs::parse();
  std::env::set_var("RUST_LOG", args.log_level);
  pretty_env_logger::init();
  let listener = create_listener(format!("{}:{}", args.ip, args.port));
  let game = Game::new(GameType::from(args.game_type.as_str()),Duration::from_secs(args.round_duration));
  let mut server: Server = Server::new(listener, game);
  server.listen();
}
