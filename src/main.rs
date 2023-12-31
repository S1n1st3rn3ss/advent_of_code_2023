#![warn(clippy::pedantic)]

pub mod day1;

pub mod day5;
pub mod day7;
pub mod day8;
pub mod day3;
pub mod day2;

use std::fs;
use std::fs::File;
use std::io::Read;
use advent_of_code_2023::*;

pub fn main() {
    let mut string: String = fs::read_to_string("input.txt").expect("");
    println!("{}", day9::part2::run(&string));
}