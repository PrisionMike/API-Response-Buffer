#![allow(unused_variables)]

use clap::Parser;
use gws::*;
// use std::io;

/// POV: You build a tank for to cache your Web API responses
/// locally for easier access.

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Web API string.
    #[clap(short, long, value_parser)]
    api: String,
    /// Number of response you want to cache (Also the total capacity of the tank, for now)
    #[clap(short, long, value_parser)]
    number: usize,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let dispenser = Dispenser::new(args.api, args.number).await;

    dbg!(dispenser);
}
