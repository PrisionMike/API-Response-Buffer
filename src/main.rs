use clap::{Arg, App};
use reqwest;
#[macro_use] extern crate rocket;

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

/*
#[tokio::main]
async fn deeznuts() {
        
}
*/

async fn updog(body : &'static str, socketadd : &str) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
       let client = reqwest::Client::new();
       let res = client.post(socketadd)
                                .body(body)
                                .send()
                                .await?;
        Ok(res)
}

#[launch]
#[tokio::main]
async fn rocket() -> _ {
    println!("Shree Ram Jaanki...");

    let matches = App::new("JHAADI")
        .version("0.1.0")
        .author("Prison Mike <su.sh2396@gmail.com>")
        .about("API Response Buffer")
        .arg(Arg::with_name("theapi")
                .short("i")
                .long("api")
                .takes_value(true)
                .help("The API whose response you want to cache"))
        .arg(Arg::with_name("capacity")
                .short("n")
                .long("size")
                .takes_value(true)
                .help("Number of Responses to cache"))
        .get_matches();

        let the_api = matches.value_of("theapi").unwrap_or("Jhingalala");
        let cap = matches.value_of("capacity").unwrap_or("hu hu");
        println!("input received: {}\n{}", the_api, cap);

        // let astr = "Mon kemoner jonmodin";
        let kyahai = rocket::build().mount("/", routes![sugma]);
        println!("{:?}",kyahai);

        println!("Let me sleep");
        let onesec = std::time::Duration::from_secs(3);
        std::thread::sleep(onesec);
        println!("Hooray and up she rises");
        let jiya = updog("elephant Camel Mongose", "http:127.0.0.1:8000");
        let kyamila = jiya.await;
        println!("{:?}",kyamila);
        kyahai
}

