use clap::{Arg, App};
use reqwest;
// use rocket::data;
use std::{collections::VecDeque, fmt};
use serde::{Deserialize};
use serde_json::Value;
// #[macro_use] extern crate rocket;


// Using 'Trait objects' here is really unnecessary given a response will always have the same type
// in a given collection. It is better we use generics. But since it SHOULD run, and it's a
// learning project, let Trait objects remain. Maybe we could try with a generic later.
// trait Hisbullah: fmt::Display{}
// impl<'de> Deserialize<'_, T: ?Sized> for dyn Hisbullah{
//         fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//         where
//                 D: serde::Deserializer<'de> {
            
//         }
// }

// impl Hisbullah for str {}   
// impl Hisbullah for u16 {}
// impl Hisbullah for String {}
#[derive(Deserialize,Debug)]
struct json_response<T> {
        maal : T
}

/*
#[get("/")]
fn ligma() -> &'static str {
        /*
        let cstr: &'static str = "Fixed response on a fixed port.\nI've confirmed
        neiter.\nAlso\n";
*/

        
//         format!("Fixed response on a fixed port.\nI've confirmed neiter.\nAlso\n{}",param)
//         */

//         "Server started.\n Waiting for an input to build resources for."
// }

// #[post("/", data = "<apiurl>")]
// fn sugma(apiurl : &str) -> String {
//         format!("We'll get back to you shortly after we run: {}",apiurl)
// }

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
#[tokio::main]
// #[rocket::main]
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
                .short("d")
                .long("data")
                .takes_value(true)
                .help("Enter the data field for the expected JSON repsonse."))
        .arg(Arg::with_name("data_type")
                .short("t")
                .long("datatype")
                .takes_value(true)
                .help("Data type of the data in the response"))
        .get_matches();

        let the_api = matches.value_of("theapi").unwrap_or("Jhingalala");
        let cap = matches.value_of("capacity").unwrap_or("hu hu");
        let dataf = matches.value_of("JSON_data_section");
        let datat = matches.value_of("data_type");
        let mut extract = false;
        println!("input received:\n
                  API:  {}\n
                  Capacity: {}\n",&the_api,&cap);
        if let Some(v) = dataf {
                println!("Data field: {}",v);
                extract = true;
                match datat {
                        Some(v) => println!("Data type: {}",v),
                        None => panic!("Provide the type of data using the -t or --datatype flag (int,str,hex,binary)")
                    }
        }


        let capint : usize = usize::from_str_radix(cap, 10).unwrap() + 1;
        
        let mut response_tank: VecDeque<String> = VecDeque::with_capacity(capint);
        for _i in 1 .. capint {
                let res = updog(the_api).await;
                response_tank.push_back(res.unwrap());
        }

        let mut data_tank: VecDeque<json_response<_>> = VecDeque::new();
        
        if extract {
                for _i in 1 .. capint {
                        let respi = response_tank.pop_front();
                        match respi {
                            None => {
                                    println!("Tank empty");
                                    break;
                            }
                            Some(v) => {
                                    let val : Value = serde_json::from_str(&v).unwrap();
                                    data_tank.push_back(val["data"]);
                            }
                        }
                        
                }
        }
        
        while !data_tank.is_empty() {
                let popdata = data_tank.pop_front().unwrap();
                println!("{}",popdata);
        }
        // let data_tank: VecDeque<>
        
        // for _i in 1 .. capint {
        //         let popped =  response_tank.pop_front();
        //         if let Some(v) = popped {
        //                 println!("{}",v);
        //                 response_tank.push_back(v);
        //         }
                
        // }
        
        // Ok(())
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

