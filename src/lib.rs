// #![allow(dead_code)]
// #![allow(unused_imports)]

/// A Dispenser never panics.

use reqwest::{self, StatusCode};
use std::{collections::VecDeque};

pub const _TEST_API_STRING: &str = "https://qrng.anu.edu.au/API/jsonI.php?length=10&type=hex16&size=2";
pub const _EMPTY_TANK_WARNING: &str = "!EMPTY_TANK";
pub const _TEST_CAPACITY: usize = 3;

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

        dispenser.fill_the_tank(capacity).await;

        dispenser
    }

    /// Fills the tank regardless of the Water quality.
    async fn _hose_the_tank(&mut self, thismuch: usize) {
        
        let client = reqwest::Client::new();

        for _ in 0 .. thismuch {
            let res = client.get(&self.webapi[..]).send().await;
            let res_output: Water = match res {
                
                Ok(v) => {
                    let response_status = v.status();
                    match v.text().await {
                        Ok(u) => {

                            match response_status {
                                reqwest::StatusCode::OK => Water { salt: Colour::Clear, data: u, response_code: None },
                                _ => Water { salt: Colour::ErrorNotOK, data: u, response_code: None }
                            }

                        }
                        Err(e) => Water{ salt: Colour::ResponseToTextError, data: e.to_string(), response_code: None},
                    }
                },
                Err(e) => Water{ salt: Colour::ReqwestError, data: e.to_string(), response_code: None},
            };

            self.tank.push_back(res_output);
        }

        // Ok(())
    }

    /// Fills the tank to the assurance that all water in it is clean. i.e. All good responses.
    async fn fill_the_tank(&mut self, thismuch: usize) -> Result<(), reqwest::Error> {
        
        let client = reqwest::Client::new();

        for _ in 0 .. thismuch {
            let mut res = client.get(&self.webapi[..]).send().await.unwrap();
            let mut drop = Water::clear_water(String::new());
            loop {
                match res.status() {
                    StatusCode::OK => {
                        drop.data = match res.text().await {
                            Ok(v) => v,
                            Err(e) => {
                                drop.salt = Colour::ResponseToTextError;
                                e.to_string()
                            }
                        };
                        break;
                    },
                    StatusCode::INTERNAL_SERVER_ERROR => {
                        // This is for this particular TEST_API_STRING. I know that it sends a 500 when there are too much
                        // requests. Normally, I think the user will have to give the input to the dispenser on what to
                        // do with this error. Seems like a DSL is in the stars...
                        std::thread::sleep(std::time::Duration::from_secs(1));
                        res = match client.get(&self.webapi[..]).send().await {
                            Ok(v) => v,
                            Err(e) => {
                                drop.salt = Colour::ReqwestError;
                                drop.data = e.to_string();
                                break;
                            }
                        };
                    },
                    _ => panic!("Server returned unexpected response.\nStatus Code: ***{}***", res.status())
                };
            }

            self.tank.push_back(drop);
        }

        Ok(())
    }

    pub fn spit(&mut self) -> Water {
        match self.tank.pop_front() {
            Some(v) => v,
            None => Water::air(),
        }
    }
    pub async fn refill_or_prune(&mut self) {
        
        if self.level_check() < self.capacity {
            let diff = self.capacity - self.level_check();
            self.fill_the_tank(diff).await;
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
/// as expected API responses. But the bigger and better reason is
/// any `pop` call from a Dequeue is automatically wrapped in an Option<T>.
/// Thus wrapping the entries themselves would be a double unwrap. Ugly.
/// Note the order of the values for the
/// `colour` function.
#[derive(Debug)]
pub struct Water {
    /// Response Type
    salt: Colour,
    /// API Response or error string
    data: String,
    response_code: Option<StatusCode>
}
impl Water {

    /// Generator for clear water.
    pub fn clear_water(_data: String) -> Water {
        Water { salt: Colour::Clear, data: _data, response_code: Some(StatusCode::OK) }
    }

    /// This function will likely consume the enum instance. Thus the name.
    pub fn condense(self) -> String {
        self.data
    }
    /// Type of the response. 0 means all good.
    pub fn colour_code(&self) -> usize {
        self.salt as usize
    }

    /// Type of response with acutal colour name.
    pub fn colour(&self) -> Colour {
        self.salt
    }
    
    pub fn status_code(&self) -> Option<reqwest::StatusCode> {
        self.response_code
    }

    /// Ersatz `None` object.
    pub fn air() -> Self {
        Water { salt: Colour::Air, data: String::new(), response_code: None }
    }
    pub fn is_air(&self) -> bool {
        match self.salt {
            Colour::Air => true,
            _ => false
        }
    }
    pub fn is_water(&self) -> bool {
        !self.is_air()
    }
}


#[derive(Debug,Clone,Copy)]
pub enum Colour {
    Air,
    Clear,
    ReqwestError,
    ResponseToTextError,
    ErrorNotOK,
    _Unknown
}
#[allow(non_snake_case)]

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn refill() {
        let mut toti = Dispenser::new(_TEST_API_STRING.to_owned(), _TEST_CAPACITY).await;
        println!("Initial level: {}",toti.level_check());
        let _ = toti.spit();
        println!("Level after spit: {}",toti.level_check());
        toti.refill_or_prune().await;
        assert_eq!(toti.level_check(), _TEST_CAPACITY)
    }

    #[tokio::test]
    async fn creation_and_spitting() {
        let mut toti = Dispenser::new(_TEST_API_STRING.to_owned(), _TEST_CAPACITY).await;
        assert_eq!(toti.level_check(), _TEST_CAPACITY);
        assert_eq!(_TEST_CAPACITY, toti.capacity);
        assert_eq!(_TEST_API_STRING,toti.webapi);
        assert!(toti.spit().is_water())
    }

    #[tokio::test]
    async fn no_air_bubbles() {
        use rand::Rng;

        let mut rng = rand::thread_rng();
        let TEST_AMT : usize = rng.gen_range(5..10);

        println!("Testing with {TEST_AMT} requests.");
        let mut toti = Dispenser::new(_TEST_API_STRING.to_owned(), TEST_AMT).await;
        let mut k = 0;
        while toti.spit().is_water() {
            k += 1;
        }
        assert_eq!(k,TEST_AMT);
    }

    #[tokio::test]
    async fn clean_water() {
        /// All Water should be clean. Responses should all be alright..
        use rand::Rng;

        let mut rng = rand::thread_rng();
        let TEST_AMT : usize = rng.gen_range(5..10);

        println!("Testing with {TEST_AMT} requests.");
        let mut toti = Dispenser::new(_TEST_API_STRING.to_owned(), TEST_AMT).await;
        let mut drop = toti.spit();
        let mut k = 0;
        while drop.is_water() {
            if drop.colour_code() == 1 {
                k += 1;
            } else {
                println!("Turbid water at: {k}\tColour: {:?}",drop.colour());
                println!("{}",drop.condense());
                break;
            }
            drop = toti.spit();
        }
        assert_eq!(k,TEST_AMT);
    }

}


