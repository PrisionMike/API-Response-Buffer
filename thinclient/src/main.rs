use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let socket = "localhost:23541";
    let mut stream = TcpStream::connect(socket).unwrap();
    // let req = "GIVE 3";
    stream.write(b"GIVE 69");
}
