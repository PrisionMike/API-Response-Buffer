#![allow(dead_code)]
#![allow(unused_imports)]

use reqwest;
use std::collections::VecDeque;

const _TEST_API_STRING: &str = "https://qrng.anu.edu.au/API/jsonI.php?length=10&type=hex16&size=2";
const _EMPTY_TANK_WARNING: &str = "!EMPTY_TANK";

#[derive(Debug)]
pub struct Dispenser {
    webapi: String,
    capacity: usize,
    tank: VecDeque<String>,
}
impl Dispenser {
    async fn fill_the_tank(&mut self) -> Result<(), reqwest::Error> {
        let client = reqwest::Client::new();

        for _ in 1..self.capacity {
            let res = client.get(&self.webapi[..]).send().await;
            let res_text = match res {
                Ok(v) => match v.text().await {
                    Ok(u) => u,
                    Err(e) => e.to_string(),
                },
                Err(e) => e.to_string(),
            };

            self.tank.push_back(res_text);
        }

        Ok(())
    }

    pub async fn new(webapi: String, capacity: usize) -> Dispenser {
        let mut dispenser = Dispenser {
            webapi,
            capacity,
            tank: VecDeque::with_capacity(capacity),
        };

        dispenser.fill_the_tank().await.unwrap();

        dispenser
    }
    pub fn spit(&mut self) -> String {
        match self.tank.pop_front() {
            Some(v) => v,
            None => _EMPTY_TANK_WARNING.to_owned(),
        }
    }
}
