use clap::{Arg, App};
use reqwest;
use std::collections::VecDeque;
use serde::{Deserialize, Serialize};
use serde_json;
#[macro_use] extern crate rocket;


// HOW TO SEND ARGUMENTS TO A ROCKET MOUNTED METHOD!!!!



#[get("/")]
fn ligma() -> &'static str {
        /*
        let cstr: &'static str = "Fixed response on a fixed port.\nI've confirmed
        neiter.\nAlso\n";

        
        format!("Fixed response on a fixed port.\nI've confirmed neiter.\nAlso\n{}",param)
        */

        "Server started.\n Waiting for an input to build resources for."
}

#[post("/", data = "<apiurl>")]
fn sugma(apiurl : &str) -> String {
        format!("We'll get back to you shortly after we run: {}",apiurl)
}

// #[get("/geralt")]
// fn ofrivia() -> (){
//         ()
// }

// async fn updog(body : &'static str, socketadd : &str) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
//        let client = reqwest::Client::new();
//        let res = client.post(socketadd)
//                                 .body(body)
//                                 .send()
//                                 .await?;
//         Ok(res)
// }

// fn sayswhat( douche: VecDeque<String>) -> () {
//         ()
// }

async fn updog(theapi : &str) -> Result<String, Box<dyn std::error::Error>> {
        // let client = reqwest::Client::new();
        let res = reqwest::get(theapi)
                                .await?
                                .text()
                                .await?;
        // println!("What's updog?");
         Ok(res)
 }

// #[launch]
// #[tokio::main]
#[rocket::main]
async fn main() -> () {
    println!("Shree Ram Jaanki...");

    let matches = App::new("JHAADI")
        .version("0.1.0")
        .author("Prison Mike <su.sh2396@gmail.com>")
        .about("API Response Buffer")
        .arg(Arg::with_name("theapi")
                .short("i")
                .long("api")
                .takes_value(true)
                .help("The API whose response you want to cache\n
                        Send the API in doublequotes."))
        .arg(Arg::with_name("capacity")
                .short("n")
                .long("size")
                .takes_value(true)
                .help("Number of Responses to cache"))
        .arg(Arg::with_name("JSON_data_section")
                .short("s")
                .long("section")
                .takes_value(true)
                .help("Enter the data field for the expected JSON repsonse."))
        .get_matches();

        let the_api = matches.value_of("theapi").unwrap_or("Jhingalala");
        let cap = matches.value_of("capacity").unwrap_or("hu hu");
        let dataf = matches.value_of("JSON_data_section");
        println!("input received:\n
                  API:  {}\n
                  Capacity: {}\n",&the_api,&cap);
        if let Some(v) = dataf {
                println!("Data field: {}",v);
        }

        let capint : usize = usize::from_str_radix(cap, 10).unwrap() + 1;
        
        let mut text_tank: VecDeque<String> = VecDeque::with_capacity(capint);
        for _i in 1 .. capint {
                let res = updog(the_api).await;
                text_tank.push_back(res.unwrap());
        }

        for _i in 1 .. capint {
                let popped =  text_tank.pop_front();
                if let Some(v) = popped {
                        println!("{}",v);
                }
        }

        // let kyahai = rocket::build().mount("/", routes![sugma,ligma]);

        // println!("Let me sleep");
        // let onesec = std::time::Duration::from_secs(3);
        // std::thread::sleep(onesec);
        // println!("Hooray and up she rises");
        // let jiya = updog("elephant Camel Mongose", "http:127.0.0.1:8000");
        // let kyamila = jiya.await;
        // println!("{:?}",kyamila);
        // kyahai.launch().await;
}

