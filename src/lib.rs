use clap;
use std::{collections::VecDeque};
use reqwest;
use chrono::prelude::*;

#[derive(Debug)]
pub struct Dispenser {
   pub tank : Option<VecDeque<String>>,
   pub api : String,
   pub capacity : usize,
   pub so_stale : Option<DateTime<Local>>,
   pub addr : String,
}
impl Dispenser {
   pub fn new( the_api: &str, cap: usize) -> Self {
      Self {
         tank: None,
         api: the_api.to_owned(),
         capacity:  cap,
         so_stale:Some(Local::now()),
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
       self.tank = Some(response_tank);

       self.so_stale = Some(Local::now());
    }

    pub fn set_addr(&mut self, addr: &str) {
       self.addr = addr.to_owned();
    }

    pub fn get_addr(&self) -> &str {
       &self.addr[..]
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