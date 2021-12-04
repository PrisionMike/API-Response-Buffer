use std::io::prelude::*;
use std::net::{TcpStream, TcpListener};

const ADDR: &str = "localhost:23541";

fn main() {
   let callback = client(b"GIVE 3");
   server(callback);
}

fn server(stream : TcpStream) {
    println!("yahan tak bhi nahi aye?");
    let localaddr = dbg!(stream.local_addr().unwrap());
    // let listener2 = TcpListener::bind("localhost:58745").unwrap();
    // dbg!(&listener2);

    // for client in listener2.incoming() {
    let listener = TcpListener::bind(localaddr).unwrap();
    dbg!(&listener);
    
    for client in listener.incoming() {
        dbg!("Koi aya?");
        let mut client = client.unwrap();
        let mut buff = Vec::with_capacity(1024);
        client.read(&mut buff).unwrap();
        println!("Humari atariya pe: {}", String::from_utf8_lossy(&buff));
    }

}

fn client(msg : &[u8])-> TcpStream {
    let mut stream = TcpStream::connect(ADDR).unwrap();
    stream.write_all(msg).unwrap();
    dbg!(stream)
}
