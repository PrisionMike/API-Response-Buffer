use clap;
 pub fn wrap_the_clap<'a> (matches: &'a clap::ArgMatches) -> ( &'a str, usize) {
    /*
    Parse the CL arguments.
    */
    let the_api = matches.value_of("theapi").unwrap_or("Jhingalala");
    let cap = matches.value_of("capacity").unwrap_or("hu hu");
    let capint: usize = usize::from_str_radix(cap, 10).unwrap() + 1;        // Parsing the capacity as usize integer.

    (the_api, capint)
 }