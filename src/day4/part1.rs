use std::collections::HashMap;
use std::ops::Add;
#[derive(Debug, Clone)]
struct Card {
    idx: i32,
    worth: i32,
    count: i32
}
// TODO: Split into parts and bench
pub fn run(input: &str) -> u64 {
    let i: Iterator<Item = (&str, &str)>  = input
        .lines()
        .map(|x| x.split_once(':').unwrap().1.trim())
        .map(|x| x.split_once('|')).into_iter();
}
pub fn parse_cards(input: Vec<String>) {
    let mut sum: i32 = 0;
    let mut map: Vec<Card> = Default::default();
    for (idx, i) in input.iter().enumerate() {
        let mut wins: Vec<&str> = vec![];
        let mut points: Vec<&str> = vec![];
        let Some((winning, card)) = i
            .trim_start_matches("Card ")
            .trim_start_matches(|c:char| c.is_digit(10) || c == ':')
            .trim()
            .split_once("|")else { todo!() };
        for j in winning.trim().split(' ') {
            if j != "" {
                wins.push(j);
            }
        }
        for j in card.trim().split(' ') {
            if j != "" {
                points.push(j);
            }
        }
        let mut power: i32 = -1;
        for i in wins.iter() {
            if points.contains(i) {
                power += 1;
            }
        }
        if power != -1 {
            let worth = 2_i32.pow(power as u32);
            sum += worth;
        }
        map.push(Card { idx: idx as i32 + 1, worth: power + 1, count: 1 })
    }
    for i in 0..map.len() {
        let count = map[i].count;
        let worth = map[i].worth;
        for j in i + 1..=i + worth as usize{
            map[j].count += 1 * count
        }
    }
    let mut sum: i32 = 0;
    for i in map {
        sum += i.count;
    }
    println!("{:#?}", sum);
}