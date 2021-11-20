use clap::{App, Arg};
use reqwest;
// use rocket::catcher::Result;
// use rocket::data;
use serde::de::{self, Deserializer, MapAccess, SeqAccess, Visitor};
use serde::Deserialize;
use serde_json::Value;
use std::{collections::VecDeque, fmt};

#[derive(Deserialize, Debug)]
struct JsonResponse2 {
    maal: Value,
}

async fn updog(theapi: &str) -> Result<String, Box<dyn std::error::Error>> {
    // let client = reqwest::Client::new();
    let res = reqwest::get(theapi).await?.text().await?;
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
            Arg::with_name("JSON_data_section")
                .short("d")
                .long("data")
                .takes_value(true)
                .help("Enter the data field for the expected JSON repsonse."),
        )
        .arg(
            Arg::with_name("data_type")
                .short("t")
                .long("datatype")
                .takes_value(true)
                .help("Data type of the data in the response"),
        )
        .get_matches();

    let the_api = matches.value_of("theapi").unwrap_or("Jhingalala");
    let cap = matches.value_of("capacity").unwrap_or("hu hu");
    let dataf = matches.value_of("JSON_data_section");
    let datat = matches.value_of("data_type");
    let mut extract = false;
    println!(
        "input received:\n
                  API:  {}\n
                  Capacity: {}\n",
        &the_api, &cap
    );
    if let Some(v) = dataf {
        println!("Data field: {}", v);
        extract = true;
        match datat {
            Some(v) => println!("Data type: {}", v),
            None => panic!(
                "Provide the type of data using the -t or --datatype flag (int,str,hex,binary)"
            ),
        }
    }

    let capint: usize = usize::from_str_radix(cap, 10).unwrap() + 1;

    let mut response_tank: VecDeque<String> = VecDeque::with_capacity(capint);
    for _i in 1..capint {
        let res = updog(the_api).await;
        response_tank.push_back(res.unwrap());
        println!()
    }

    let mut data_tank: VecDeque<JsonResponse2> = VecDeque::new();

    if extract {
        for _i in 1..capint {
            let respi = response_tank.pop_front();
            match respi {
                None => {
                    println!("Tank empty");
                    break;
                }
                Some(v) => {
                    dbg!(v.clone());
                    let val: Value = serde_json::from_str(&v[..]).unwrap();
                    let malvalue = &val[dataf.unwrap()].to_owned();
                    data_tank.push_back(JsonResponse2 {
                        maal: malvalue.to_owned(),
                    });
                }
            }
        }
    }
    while !data_tank.is_empty() {
        let front_item = data_tank.pop_front();
        // dbg!(&front_item);
        match front_item {
            Some(v) => {
                    println!("Ishank {:?}", v);
            }
            None => println!("Sharma"),
        };
    }
    // let data_tank: VecDeque<>

    // for _i in 1 .. capint {
    //         let popped =  response_tank.pop_front();
    //         if let Some(v) = popped {
    //                 println!("{}",v);
    //                 response_tank.push_back(v);
    //         }

    // }
}
