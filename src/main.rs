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
    #[clap(short, long, value_parser)]
    number: usize,
}

fn main() {
    let args = Args::parse();

    let dispenser = Dispenser::new(args.api, args.number);

    dbg!(dispenser)
}
