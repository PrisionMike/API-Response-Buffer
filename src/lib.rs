use clap;
use std::{collections::VecDeque};
use reqwest;
 pub fn wrap_the_clap<'a> (matches: &'a clap::ArgMatches) -> ( &'a str, usize) {
    /*
    Parse the CL arguments.
    */
    let the_api = matches.value_of("theapi").unwrap_or("Jhingalala");
    let cap = matches.value_of("capacity").unwrap_or("hu hu");
    let capint: usize = usize::from_str_radix(cap, 10).unwrap() + 1;        // Parsing the capacity as usize integer.

    (the_api, capint)
 }

 pub async fn charge_the_tank(the_api: &str, cap: usize) -> VecDeque<String> {
    
   let mut response_tank: VecDeque<String> = VecDeque::with_capacity(cap);
    for _i in 1..cap {
        /*
        The code is blocking rn. What we have to do is to make multiple async calls to the API
        (kinda like putting in multiple hoses in the tank) to speed up the fetch.
        That is a great piece of code, yet to be written. Untill then..
        */
        let res = updog(the_api).await;
      //   let res_string = res.unwrap();
        response_tank.push_back(res.unwrap());
    }
    response_tank
 }

 async fn updog(theapi: &str) -> Result<String, Box<dyn std::error::Error>> {
   /*
   The function to return the result of the given API. Simple wrapper around the reqwest, essentially.
   */

   let res = reqwest::get(theapi).await?.text().await?;
   Ok(res)
}