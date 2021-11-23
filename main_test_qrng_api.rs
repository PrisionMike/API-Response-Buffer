// use reqwest;
// use std::collections::HashMap;
use serde::Deserialize;


#[tokio::main]
async fn giraffe(len : u16) -> Result<String, Box<dyn std::error::Error>> {

    const MAXLEN: usize = 32;   // MAX VALUE FOR DESERIALIZE

    #[derive(Deserialize,Debug)]
    struct QrngResponse{
        #[serde(rename = "type")]
        type_ : String,
        length : u16,
        // data : [u16; MAXLEN],    // for uint16
        data : [String; MAXLEN],    // for hex16
        success: bool
    }

    // Calculating the number of QRNG calls:
    let blocks = len / 512;

    let mut thebiggerint = String::from(r#""#);
    let apiurl = "https://qrng.anu.edu.au/API/jsonI.php?length=";
    let hexsize = 16;

    // let reqstr = format!("{}{}&type=uint16",apiurl,MAXLEN.to_string());      // for uint16
    let reqstr = format!("{}{}&type=hex16&size={}",&apiurl,MAXLEN.to_string(),&hexsize);   // for hex16

    for _j in 0 .. blocks {

        println!("Requested API: {}",reqstr);

        let resp = reqwest::get(&reqstr)
                .await?;
                // .json::<QrngResponse>()
                // .text()
                // .await?;
        println!("{}", resp);

        // let mut i = 0; 
        // while i < MAXLEN {
        //     let b = u128::from_str_radix(&resp.data[i],16).unwrap();
        //     // let a = format!("{}",&resp.data[i]);
        //     // thebiggerint = format!("{}{}",thebiggerint,&a);
        //     thebiggerint = format!("{}{}",thebiggerint,&b);
        //     i += 1;
        // }
    }    

    // println!("\nThe bigger int for ya:\n{}",thebiggerint);
    
    Ok(thebiggerint)
}

fn main() {
    let suyash = giraffe(512).unwrap();
    println!("Suyash says: {}",suyash);

}
