//use clap::{App, Arg};
use structopt::StructOpt;
use reqwest;
use tokio;
use jhaadi::*;
use chrono::prelude::*;
use std::collections::HashMap;
use std::io;

/// API Documentation https://qrng.anu.edu.au/contact/api-documentation/
/// To run the program use cargo run -- -n [value] or got to ../API-Response-Buffer/target/debug/jhaadi
/// and run ./jhaadi -n [value]

#[derive(Debug, StructOpt)]
#[structopt(name = "jhaadi", about="API Response Buffer.")]
struct Args {
    #[structopt(name="capacity", short="n", long="size", default_value="10")]
    capacity: String,
}

// async fn get_response(api_call_input: String) -> String {
//     let res = reqwest::get(api_call).await?.text().await?;
//     match res {
//         Ok(res) => res,
//         Err(err) => panic!(),
//     }
// }


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("App is running...");  // Just to make sure the main ran. :P

    let arg = Args::from_args();
    println!("{:?}", arg);
    
    let api_call = format!("https://qrng.anu.edu.au/API/jsonI.php?length={}&type=uint16&size=12", arg.capacity);
    println!("{}", api_call);

    // let res = get_response(api_call);

    // let res = reqwest::get(api_call).await?.text().await?;
    // match res {
    //     Ok(json) => println!("{:?}", json),
    //     Err(_) => println!("{:?}", Err),
    // }
    

    let res = reqwest::get(api_call).await?.json().await?;
    Ok(println!("res = {:?}", res))


    // println!("res = {:?}", res);


    // Ok(())
}





// CLAP: Command Line Argument Parser.
// let matches = App::new("jhaadi")
// .version("0.1.0")
// .author("Prison Mike <su.sh2396@gmail.com>")
// .about("API Response Buffer")
// .arg(
//     Arg::with_name("capacity")
//         .short("n")
//         .long("size")
//         .takes_value(true)
//         .help("Number of Responses to cache"),
// )
// .get_matches();

// println!("{:?}", "matches");
// let (the_api, capint) = wrap_the_clap(&matches); 
    
// let mut disone = Dispenser::new(the_api, capint);

// println!("Request received for:\nAPI: {}\nCapacity: {}", disone.api, disone.capacity);

// // let mut response_tank = charge_the_tank(the_api, capint).await;
// disone.charge_the_tank().await;

// let gentime = Local::now();
// println!("Tank filled at {:?}",gentime);

// let listeny_port = "23541";
// let fulladd = format!("localhost:{}",listeny_port);

// disone.set_addr(&fulladd[..]);
// println!("All set at {}\nWould you like to activate?",disone.get_addr());
// let mut input = String::new();
// io::stdin().read_line(&mut input).expect("Error reading from stdin");

// if input.to_lowercase() == "yes\r\n" {
//     disone.deploy_engaged();
//     println!("Server deployed. You probably can't do anything in this terminal now")
// } else if input.to_lowercase() == "no\r\n" {
//     println!("Alright!")
// } else {
//     println!("The answer was supposed to be either yes or no. Fuck off for now.")
// }