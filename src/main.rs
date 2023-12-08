#![warn(clippy::pedantic)]

mod day1;

mod day5;
mod day7;
mod day8;
mod day3;

use std::fs::File;
use std::io::Read;

fn main() {
    let mut string: String = String::new();
    let i = File::open("input.txt")
        .expect("TODO: panic message")
        .read_to_string(&mut string);
    println!("{}", day8::part1::run(&string));
}