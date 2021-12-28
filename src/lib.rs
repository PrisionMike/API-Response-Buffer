use clap;
use std::collections::VecDeque;
use reqwest;
use chrono::prelude::*;
use std::net::SocketAddr;
use actix_web::{Responder,HttpResponse};

/*
- VecDeque is the standard way to implement any Que in Rust. It stands fro a Double Ended Queue.
Here we need to use it as a simple linear single ended queue, thus only push_back() and pop_front().

- chrono: date/time stamp operations.

- reqwest: making GET requests to the target Web API.

- clap: command line argument parsing

*/

#[derive(Debug)]
pub struct Dispenser {
   pub name : String,
   pub tank : VecDeque<String>,
   pub api : String,
   pub capacity : usize,                              // Maximum number of responses the tank can hold.
   pub so_stale : Option<DateTime<Local>>,            // Time stamp of the last response.
   pub addr : Option<SocketAddr>,                                 // Server address. Resource address in future methods.
}
impl Dispenser {

   /*
   Future improvement:
      - Return errors for important functions.
   
   */
   pub fn new( naam: String, the_api: &str, cap: usize) -> Self {
      Self {
         name: naam,
         tank: VecDeque::with_capacity(cap),
         api: the_api.to_owned(),
         capacity:  cap,                        
         so_stale: Some(Local::now()),          
         addr : None,               
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

   pub fn set_addr(&mut self, addr: SocketAddr) {
       self.addr = Some(addr);
    }

   pub fn get_addr(&self) -> &Option<SocketAddr> {
       &self.addr
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

pub async fn homepage() -> impl Responder {
   HttpResponse::Ok().body("The dispenser will be with you shortly.")
}