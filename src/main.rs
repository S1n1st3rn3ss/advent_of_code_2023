#![warn(clippy::pedantic)]

mod day1;
mod day2;
mod day3;

mod day4;
mod day5;

use std::fs::File;
use std::io::Read;
use crate::day4::parse_cards;

fn main() {
    let mut string: String = String::new();
    let i = File::open("input.txt")
        .expect("TODO: panic message")
        .read_to_string(&mut string);
    println!("{}", day5::part1::run(&string));
}