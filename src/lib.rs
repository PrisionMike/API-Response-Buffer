use clap;
 pub fn wrap_the_clap<'a> (matches: &'a clap::ArgMatches) -> ( &'a str, usize, Option<&'a str>, Option<&'a str>, bool) {
    /*
    Parse the CL arguments.
    */
    let the_api = matches.value_of("theapi").unwrap_or("Jhingalala");
    let cap = matches.value_of("capacity").unwrap_or("hu hu");
    let dataf = matches.value_of("JSON_data_section");
    let datat = matches.value_of("data_type");

    let mut extract = false;        // Flag to know if the last 2 arguments (for extracting JSON field) have been provided.

    /*
    Optional parameter parsing.
    */
    if let Some(v) = dataf {
        println!("Data field: {}", v);
        extract = true;
        match datat {
            /*
            So far this field has not yet been required in the application.
            */
            Some(v) => println!("Data type: {}", v),
            None => (),
        }
    }

    let capint: usize = usize::from_str_radix(cap, 10).unwrap() + 1;        // Parsing the capacity as usize integer.

    (the_api, capint, dataf, datat, extract)
 }