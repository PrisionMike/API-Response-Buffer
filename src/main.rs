use clap::{Arg, App};
#[macro_use] extern crate rocket;

#[get("/")]
fn ligma() -> &'static str {
        "Fixed response on a fixed port.\nI've confirmed
        neiter."
}


#[launch]
fn rocket() -> _ {
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

        rocket::build().mount("/", routes![ligma])
}
