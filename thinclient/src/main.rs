use std::io::prelude::*;
use std::net::{TcpStream, TcpListener};

const ADDR: &str = "localhost:23541";

fn main() {
   let server_handler = std::thread::spawn(server);
   
   client(b"GIVE 2");
   server_handler.join().unwrap()
}

fn server() {
    let listener = TcpListener::bind(ADDR).unwrap();

    for client in listener.incoming() {
        let mut client = client.unwrap();
        let mut buff = Vec::with_capacity(1024);
        client.read_to_end(&mut buff).unwrap();
        println!("Humari atariya pe: {}", String::from_utf8_lossy(&buff));
    }
}

fn client(msg : &[u8]) {
    let mut stream = TcpStream::connect(ADDR).unwrap();
    stream.write_all(msg).unwrap();
}
