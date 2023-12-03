#![warn(clippy::pedantic)]

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

// fn main() {
//     let mut string: String = String::new();
//     File::open("input.txt").expect("TODO: panic message").read_to_string(&mut string).unwrap();
//     let mut str_arr = string.lines().map(str::to_string);
//     let mut sum: i32 = 0;
//     for i in str_arr {
//         sum += calibration_value(i);
//     }
//     println!("{sum}", );
// }
fn calibration_value(input: String) -> i32 {
    let mut temp_arr: Vec<i32> = vec![];

    for i in 0..input.len() {
        let j = &input[i..input.len()];
        if j.starts_with('1') || j.starts_with("one") { temp_arr.push(1); }
        if j.starts_with('2') || j.starts_with("two") { temp_arr.push(2); }
        if j.starts_with('3') || j.starts_with("three") { temp_arr.push(3); }
        if j.starts_with('4') || j.starts_with("four") { temp_arr.push(4); }
        if j.starts_with('5') || j.starts_with("five") { temp_arr.push(5); }
        if j.starts_with('6') || j.starts_with("six") { temp_arr.push(6); }
        if j.starts_with('7') || j.starts_with("seven") { temp_arr.push(7); }
        if j.starts_with('8') || j.starts_with("eight") { temp_arr.push(8); }
        if j.starts_with('9') || j.starts_with("nine") { temp_arr.push(9); }
    }
    let double_digit = temp_arr[0] * 10 + &temp_arr[temp_arr.len() - 1];
    double_digit
}