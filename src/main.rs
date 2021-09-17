use clap::{Arg, App};

fn main() {
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

        let position = matches.value_of("naam").unwrap();
        println!("input received: {}", position);


}
