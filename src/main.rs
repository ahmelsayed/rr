// Auther: Ahmed ElSayed
// Date: 8-31-2014
extern crate rr;

use std::io;
use std::os;
use rr::parser;

fn main() {
    let rr_options = rr::parser::parse(os::args());
    match rr_options.mode {
        parser::Help => println!("help"),
        _ => println!("anything else")
    }
}