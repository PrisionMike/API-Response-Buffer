use clap::{App, Arg};
use jhaadi::wrap_the_clap;
use jhaadi::charge_the_tank;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
struct JsonResponse2 {
    /*
    The structure to store any arbitrary Json Response value. This may not be required anymore.
     */

    maal: Value,
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

    let mut response_tank = charge_the_tank(the_api, capint).await;
    dbg!(&response_tank);
}
