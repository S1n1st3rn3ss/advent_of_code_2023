#![warn(clippy::pedantic)]


mod day3;

use std::fs::File;
use std::io::Read;

fn main() {
    let mut string: String = String::new();
    File::open("input.txt").expect("TODO: panic message").read_to_string(&mut string).unwrap();
    // let mut str_arr = string.lines().map(str::to_string).collect();

    let i = day3::parse_field(string);
    day3::find_symbols(i);
}