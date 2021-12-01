use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let socket = "localhost:23541";
    let mut stream = TcpStream::connect(socket).unwrap();
    stream.write(b"GIVE 3");
}
