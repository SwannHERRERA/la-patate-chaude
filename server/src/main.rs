use std::net::{SocketAddr, TcpListener};
use std::io::Read;

fn main() {
  println!("Hello, world!");
  let addr = SocketAddr::from(([127, 0, 0, 1], 7878));
  let listener = TcpListener::bind(addr);

  let listener = match listener {
    Ok(l) => l,
    Err(err) => panic!("Cannot listen on port : {err:?}")
  };

  for message in listener.incoming() {
    println!("message={message:?}");
    let mut message = message.unwrap();
    let mut v = Vec::<u8>::new();
    let _size_read = message.read_to_end(&mut v);
    let str = String::from_utf8_lossy(&v);
    println!("{str:?}");

    if str == "Hello" {
      println!("recived hello");
    }
  }

}