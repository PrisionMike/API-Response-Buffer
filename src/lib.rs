use clap;
use std::collections::VecDeque;
use reqwest;
use chrono::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

/*
- VecDeque is the standard way to implement any Que in Rust. It stands fro a Double Ended Queue.
Here we need to use it as a simple linear single ended queue, thus only push_back() and pop_front().

- chrono: date/time stamp operations.

- reqwest: making GET requests to the target Web API.

- clap: command line argument parsing

*/

#[derive(Debug)]
pub struct Dispenser {
   pub tank : VecDeque<String>,
   pub api : String,
   pub capacity : usize,                              // Maximum number of responses the tank can hold.
   pub so_stale : Option<DateTime<Local>>,            // Time stamp of the last response.
   pub addr : String,                                 // Server address. Resource address in future methods.
}
impl Dispenser {

   /*
   Future improvement:
      - Return errors for important functions.
   
   */
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

      /*
           The code is blocking rn. What we have to do is to make multiple async calls to the API
           (kinda like putting in multiple hoses in the tank) to speed up the fetch.
           That is a great piece of code, yet to be written. Untill then..
      */

      self.so_stale = Some(Local::now());

       for _i in self.tank.len() .. self.capacity-1 {
           let res = updog(&self.api[..]).await;
           self.tank.push_back(res.unwrap());
       }
        
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
      println!("handle_them starts");
      let mut buffer = [0; 16];
      stream.read(&mut buffer).unwrap();

      let req = String::from_utf8_lossy(&buffer[..]);
      println!("Request: {}",req);        // ==> GIVE 5
      dbg!(&req);

      println!("============ Allow me to answer it ===========");

      let req_words: Vec<&str> = req.split(' ').collect();             // [GIVE],[5]
      let command = req_words[0];                                 // GIVE
      let nullchar = String::from_utf8_lossy(&[0]);      // The character used to instantiate an empty buffer. 
      let num = req_words[1].split(nullchar.as_ref()).next().unwrap();    // "5"
      let numeric = u8::from_str_radix(num,10).unwrap();  // 5
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
                  // stream.flush().unwrap();
                  let mut stream2 = TcpStream::connect("localhost:58745").unwrap();

                  dbg!(&stream2);
                  stream2.write("<RESPONSE>".as_bytes()).unwrap();
                  stream2.write(v.as_bytes()).unwrap();
                  stream2.write("</RESPONSE>".as_bytes()).unwrap();

                  dbg!(&stream);
                  stream.write("<RESPONSE>".as_bytes()).unwrap();
                  stream.write(v.as_bytes()).unwrap();
                  stream.write("</RESPONSE>".as_bytes()).unwrap();
               }

               None => {
                  println!("It was None.");
                  stream.write("</RESPONSE>".as_bytes()).unwrap();
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
    let capint: usize = usize::from_str_radix(cap, 10).unwrap();        // Parsing the capacity as usize integer.

    (the_api, capint)
 }

 async fn updog(theapi: &str) -> Result<String, Box<dyn std::error::Error>> {
   /*
   The function to return the result of the given API. Simple wrapper around the reqwest, essentially.
   */

   let res = reqwest::get(theapi).await?.text().await?;
   Ok(res)
}