use clap::{Arg, App};

fn main() {
    println!("Shree Ram Jaanki...");

    let matches = App::new("JHAADI")
        .version("0.1.0")
        .author("Suyash Shandilya <su.sh2396@gmail.com>")
        .about("API Response Buffer")
        .arg(Arg::with_name("naam")
                .short("k")
                .long("kbaathogayi")
                .takes_value(true)
                .help("Just say 'Peechhe' and get it over with"))
        .get_matches();

        let position = matches.value_of("naam").unwrap_or("neeche moot de");
        println!("input received: {}", position);


}
