use clap::{App, Arg};
use jhaadi::wrap_the_clap;
use reqwest;
use serde::Deserialize;
use serde_json::Value;
use std::{collections::VecDeque};

#[derive(Deserialize, Debug)]
struct JsonResponse2 {
    /*
    The structure to store any arbitrary Json Response value. This may not be required anymore.
     */

    maal: Value,
}

async fn updog(theapi: &str) -> Result<String, Box<dyn std::error::Error>> {
    /*
    The function to return the result of the given API. Simple wrapper around the reqwest, essentially.
    */

    let res = reqwest::get(theapi).await?.text().await?;
    Ok(res)
}

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

    let mut response_tank: VecDeque<String> = VecDeque::with_capacity(capint);
    for _i in 1..capint {
        /*
        The code is blocking rn. What we have to do is to make multiple async calls to the API
        (kinda like putting in multiple hoses in the tank) to speed up the fetch.
        That is a great piece of code, yet to be written. Untill then..
        */
        let res = updog(the_api).await;
        response_tank.push_back(res.unwrap());
        println!()
    }
    dbg!(&response_tank);
}
