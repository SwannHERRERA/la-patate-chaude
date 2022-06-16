use log::{info, debug};
use shared::message::Message;

pub fn handle_message(message: Message) {
    info!("Hello: {:?}", message);
    match message {
      Message::Hello => {
        let answer = Message::Welcome { version: 1 };
        debug!("Answer: {:?}", answer);
        // send answer
      }
      _ => {}
    }
}
