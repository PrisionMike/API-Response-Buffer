use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::net::Shutdown;

fn main() {
    let socket = "localhost:23541";
    let listener = TcpListener::bind(&socket).unwrap();
    let streamiterator = listener.incoming();
    let mut stream = TcpStream::connect(&socket).unwrap();
    stream.write(b"GIVE 1");
    // stream.shutdown(Shutdown::Write);
    dbg!(&listener);
    dbg!(&streamiterator);
    for listensi in streamiterator {
        println!("alpha");
        let mut list_stream = listensi.unwrap();
        println!("beta");
        let mut buf = [0; 1024];
        list_stream.read(&mut buf).unwrap();
        dbg!(&buf);
        println!("Humri atariya pe: {}",String::from_utf8_lossy(&buf[..]));
    }
}
