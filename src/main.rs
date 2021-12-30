use clap::{App, Arg};
use jhaadi::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use actix_web::{web, App as Actixapp, HttpServer};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Shree Ram Jaanki...");            // Just to make sure the main ran. :P

    /*
    The CLAP part. CLAP: Command Line Argument Parser.
    You define what all flags your application can accept, assign each of them a name to be used.
    Mind you, all input is treated as string, and as it suits the best - &str particularly.
    Option<&str> to allow for ommission.
    Read help to know what each would do. CLAP automatically makes the --help functionality too.
    Drew it out from the Rust cookbook. Great reference.
    */
    let matches = App::new("jhaadi")
        .version("0.1.0")
        .author("Prison Mike <su.sh2396@gmail.com>")
        .about("API Response Buffer")
        .arg(
            Arg::with_name("theapi")
                .short("i")
                .long("api")
                .takes_value(true)
                .help(
                    "The API whose response you want to cache\n
                        Send the API in doublequotes.",
                ),
        )
        .arg(
            Arg::with_name("capacity")
                .short("n")
                .long("size")
                .takes_value(true)
                .help("Number of Responses to cache"),
        )
        .arg(
            Arg::with_name("dispenser_name")
            .short("m")
            .long("name")
            .takes_value(true)
            .help(
                "Give a title to the dispenser for referencing.
                This name will be a part of the local API text so chose as per convenience.
                Maybe put something as small as a number. e.g. \"1\""
            )
        )
        .get_matches();
    
    let (the_api, capint) = wrap_the_clap(&matches);
    println!("Clap wrapped"); 
    
    let naam1 = String::from("/d1");
    let mut disone = Dispenser::new(naam1, the_api, capint);
    println!("Disp being built for:\nAPI: {}\nCapacity: {}\n naam: {}", disone.api, disone.capacity, disone.name);

    disone.charge_the_tank().await;
    println!("Tank charged.");

    let listeny_port : u16 = 23541;
    let sockaddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0,  1)), listeny_port);

    disone.set_addr(sockaddr);
    // println!("{}",disone.get_addr());
    
    HttpServer::new( move || {
        Actixapp::new()
            .service( testurlflag )
            .service(homepage )
            // .service( testurlflag )
    })
    .bind(&disone.addr.unwrap())?
    .run()
    .await
}
