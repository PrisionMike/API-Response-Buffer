use clap::{App, Arg};
use jhaadi::*;
use chrono::prelude::*;
use std::io;

#[tokio::main]
async fn main() -> () {
    println!("Shree Ram Jaanki...");            // Just to make sure the main ran. :P

    /*
    The CLAP part. CLAP: Command Line Argument Parser.
    You define what all flags your application can accept, assign each of them a name to be used.
    Mind you, all input is treated as string, and as it suits the best - &str particularly.
    Option<&str> to allow for ommission.
    Read help to know what each would do. CLAP automatically makes the --help functionality too.
    Drew it out from the Rust cookbook. Great reference.
    */
    let matches = App::new("JHAADI")
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
        .get_matches();
    
    let (the_api, capint) = wrap_the_clap(&matches); 
    
    let mut disone = Dispenser::new(the_api, capint);

    println!("Request received for:\nAPI: {}\nCapacity: {}", disone.api, disone.capacity);

    // let mut response_tank = charge_the_tank(the_api, capint).await;
    disone.charge_the_tank().await;

    let gentime = Local::now();
    println!("Tank filled at {:?}",gentime);

    let listeny_port = "23541";
    let fulladd = format!("localhost:{}",listeny_port);

    disone.set_addr(&fulladd[..]);
    println!("All set at {}\nWould you like to activate?",disone.get_addr());
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading from stdin");
    
    if input.to_lowercase() == "yes\r\n" {
        disone.deploy_engaged();
        println!("Server deployed. You probably can't do anything in this terminal now")
    } else if input.to_lowercase() == "no\r\n" {
        println!("Alright!")
    } else {
        println!("The answer was supposed to be either yes or no. Fuck off for now.")
    }
    
}
