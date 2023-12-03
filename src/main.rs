#![warn(clippy::pedantic)]


mod day3;
mod day2;

use std::fs::File;
use std::io::Read;

fn main() {
    let mut string: String = String::new();
    File::open("input.txt").expect("TODO: panic message").read_to_string(&mut string).unwrap();
    let mut str_arr = string.lines().map(str::to_string).collect();
    let b = day2::parse_game(str_arr);
    day2::check_possibility(b);
    let i = day3::parse_field(string);
    // let j = day3::find_symbols(i);
    day3::find_numbers(i);
    // day3::find_gears(string.clone());
}