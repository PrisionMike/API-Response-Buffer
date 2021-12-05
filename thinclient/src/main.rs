use std::io::prelude::*;
use std::net::{TcpStream, TcpListener, SocketAddr, IpAddr, Ipv4Addr};

const ADDR: &str = "localhost:23541";

fn main() {
   let callback = client(b"GIVE 3");
   let addr = callback.local_addr().unwrap();
   let addr1 = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127,0,0,1)),58745);
   std::thread::spawn(move || server(addr1));
//    let server2 = std::thread::spawn(server(addr1));
   server(addr);
   
}

fn server(localaddr : SocketAddr) {
    println!("yahan tak bhi nahi aye?");
    // let listener2 = TcpListener::bind("localhost:58745").unwrap();
    // dbg!(&listener2);

    // for client in listener2.incoming() {
    let listener = TcpListener::bind(localaddr).unwrap();
    
    for client in listener.incoming() {
        dbg!(&listener);
        let mut client = client.unwrap();
        let mut buff = Vec::with_capacity(1024);
        client.read(&mut buff).unwrap();
        println!("Humari atariya pe: {}", String::from_utf8_lossy(&buff));
        dbg!(&buff);
    }

}

fn client(msg : &[u8])-> TcpStream {
    let mut stream = TcpStream::connect(ADDR).unwrap();
    stream.write_all(msg).unwrap();
    dbg!(stream)
}
