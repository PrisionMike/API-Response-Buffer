use clap;
use std::borrow::Borrow;
use std::collections::VecDeque;
use reqwest;
use chrono::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Dispenser {
   pub tank : VecDeque<String>,
   pub api : String,
   pub capacity : usize,
   pub so_stale : Option<DateTime<Local>>,
   pub addr : String,
}
impl Dispenser {
   pub fn new( the_api: &str, cap: usize) -> Self {
      Self {
         tank: VecDeque::with_capacity(cap),
         api: the_api.to_owned(),
         capacity:  cap,
         so_stale: Some(Local::now()),
         addr : String::from(""),
      }
   }

   pub async fn charge_the_tank(&mut self) {
    
      let mut response_tank: VecDeque<String> = VecDeque::with_capacity(self.capacity);
       for _i in 1..self.capacity {
           /*
           The code is blocking rn. What we have to do is to make multiple async calls to the API
           (kinda like putting in multiple hoses in the tank) to speed up the fetch.
           That is a great piece of code, yet to be written. Untill then..
           */
           let res = updog(&self.api[..]).await;
         //   let res_string = res.unwrap();
           response_tank.push_back(res.unwrap());
       }
       self.tank = response_tank;

       self.so_stale = Some(Local::now());
    }

    pub fn set_addr(&mut self, addr: &str) {
       self.addr = addr.to_owned();
    }

    pub fn get_addr(&self) -> &str {
       &self.addr[..]
    }

    pub fn deploy_engaged(&mut self) {
      let listener = TcpListener::bind(&self.addr).unwrap();
      for stream in listener.incoming() {
         let stream = stream.unwrap();
         println!("You're visiting the server for {} API.\n
                  last cached at: {:?}",self.api,self.so_stale.unwrap());
         self.handle_them(stream);
      }
    }
   fn handle_them(&mut self, mut stream: TcpStream) {
      let mut buffer = [0; 16];
      stream.read(&mut buffer).unwrap();

      let req = String::from_utf8_lossy(&buffer[..]);
      dbg!(&req);
      for c in req.chars() {
         println!("The chars: {}",c);
      }
      println!("Request: {}",req);

      println!("============ Allow me to answer it ===========");

      let req_words: Vec<&str> = req.split(' ').collect();
      let command = req_words[0];
      let nullchar = String::from_utf8_lossy(&[0]);
      let nullcharborrowed = nullchar.as_ref();
      let mut rest = req_words[1].split(nullcharborrowed);
      let num = rest.next().unwrap();
      let numeric = u8::from_str_radix(num,10).unwrap();
      dbg!(&req_words);
      dbg!(&command);
      dbg!(&num);
      if command == "GIVE" {
         println!("Valid response.");
         
         for i in 0 .. numeric {
            println!("{} pop.",&i);
            let resp = self.tank.pop_front();
            match resp {
               Some(v) => {
                  println!("It was some {}",&v);
                  stream.flush();
                  stream.write("<RESPONSE>".as_bytes());
                  stream.write(v.as_bytes());
                  stream.write("</RESPONSE>".as_bytes());
               }

               None => {
                  println!("It was None.");
                  stream.write("</RESPONSE>".as_bytes());
               }
            }
         }
      }
   }
}

 pub fn wrap_the_clap<'a> (matches: &'a clap::ArgMatches) -> ( &'a str, usize) {
    /*
    Parse the CL arguments.
    */
    let the_api = matches.value_of("theapi").unwrap_or("Jhingalala");
    let cap = matches.value_of("capacity").unwrap_or("hu hu");
    let capint: usize = usize::from_str_radix(cap, 10).unwrap() + 1;        // Parsing the capacity as usize integer.

    (the_api, capint)
 }

 async fn updog(theapi: &str) -> Result<String, Box<dyn std::error::Error>> {
   /*
   The function to return the result of the given API. Simple wrapper around the reqwest, essentially.
   */

   let res = reqwest::get(theapi).await?.text().await?;
   Ok(res)
}