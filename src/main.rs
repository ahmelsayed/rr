// Auther: Ahmed ElSayed
// Date: 8-31-2014
extern crate rr;

use std::io;
use std::os;
use rr::parser;
use rr::store;

fn main() {
    let rr_options = rr::parser::parse(os::args());
    match rr_options.mode {
        parser::Help => show_help(),
        parser::Get => store::get(&rr_options.key),
        parser::Set => store::set(&rr_options.key, &rr_options.value),
        parser::Delete => store::delete(&rr_options.key),
        parser::ShowAll => store::showall()
    }
}

fn show_help() {
    println!("get:");
    println!("\trr {{key}}");
    println!("set:");
    println!("\trr {{key}} {{value}}");
    println!("commands:");
    println!("\trr ?");
    println!("\trr delete {{key}}");
    println!("\trr all");
}