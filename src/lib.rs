// #![allow(dead_code)]
// #![allow(unused_imports)]

use reqwest;
use std::collections::VecDeque;

const _TEST_API_STRING: &str = "https://qrng.anu.edu.au/API/jsonI.php?length=10&type=hex16&size=2";
const _EMPTY_TANK_WARNING: &str = "!EMPTY_TANK";
const _TEST_CAPACITY: usize = 3;

#[derive(Debug)]
pub struct Dispenser {
    webapi: String,
    capacity: usize,
    tank: VecDeque<Water>,
}
impl Dispenser {

    pub async fn new(webapi: String, capacity: usize) -> Dispenser {
        let mut dispenser = Dispenser {
            webapi,
            capacity,
            tank: VecDeque::with_capacity(capacity),
        };

        dispenser.fill_the_tank(capacity).await.unwrap();

        dispenser
    }

    async fn fill_the_tank(&mut self, thismuch: usize) -> Result<(), reqwest::Error> {
        let client = reqwest::Client::new();

        for _ in 0 .. thismuch {
            let res = client.get(&self.webapi[..]).send().await;
            let res_output: Water = match res {
                Ok(v) => match v.text().await {
                    Ok(u) => Water::Good(u),
                    Err(e) => Water::BadResponseToTextError(e.to_string()),
                },
                Err(e) => Water::BadReqwestError(e.to_string()),
            };

            self.tank.push_back(res_output);
        }

        Ok(())
    }

    pub fn spit(&mut self) -> String {
        match self.tank.pop_front() {
            Some(v) => v,
            None => _EMPTY_TANK_WARNING.to_owned(),
        }
    }
    pub async fn refill_or_prune(&mut self) {
        
        if self.level_check() < self.capacity {
            let diff = self.capacity - self.level_check();
            self.fill_the_tank(diff).await.unwrap();
        } else if self.level_check() > self.capacity {
            let diff = self.level_check() - self.capacity;
            for _ in 0 .. diff {
                self.spit();
            }
        }
    }

    fn level_check(&self) -> usize {
        self.tank.len()
    }
}

/// The reason I am not going for Option<> as entries and instead having
/// a specialized struct is to embrace any API response errors as nicely
/// as expected API responses.
/// /// Type of entry.
/// 0 => Null value
/// 1 => Good API Response.
/// 2 => Bad API Response.
enum Water{
    AIR,
    Good(String),
    BadReqwestError(String),
    BadResponseToTextError(String),
    BadUnknown(String)
}

#[cfg(test)]
mod tests {
    use crate::{Dispenser, _TEST_API_STRING, _TEST_CAPACITY};

    #[tokio::test]
    async fn refill_test() {
        let mut toti = Dispenser::new(_TEST_API_STRING.to_owned(), _TEST_CAPACITY).await;
        println!("Initial level: {}",toti.level_check());
        let _ = toti.spit();
        println!("Level after spit: {}",toti.level_check());
        toti.refill_or_prune().await;
        assert_eq!(toti.level_check(), _TEST_CAPACITY)
    }

    #[tokio::test]
    async fn creation_works() {
        let mut toti = Dispenser::new(_TEST_API_STRING.to_owned(), _TEST_CAPACITY).await;
        assert_eq!(toti.level_check(), _TEST_CAPACITY);
        assert_eq!(_TEST_CAPACITY, toti.capacity);
        assert_eq!(_TEST_API_STRING,toti.webapi);
        assert!(!toti.spit().is_empty())
    }

}
