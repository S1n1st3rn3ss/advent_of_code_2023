use std::str::Chars;
use hashbrown::HashMap;
pub fn run(input: &str) -> u64 {
    let mut input = parse(input);
    input
}
#[derive(Debug, Clone)]
struct Node {
    left: String,
    right: String
}
fn parse(input: &str) -> u64 {
    let lines: Vec<String> = input
        .trim()
        .lines()
        .map(|x| x.to_owned())
        .collect();
    // True = Right, False = Right
    let inputs: Vec<bool> = lines[0].chars().map(|x| x == 'R').collect();
    let first: String = lines[2].split_once('=').unwrap().0.trim().to_owned();
    let path: _ = lines.iter().skip(2)
        .map(|x| x.split_once('=').unwrap())
        .map(|(start, end)| (start, end.split_once(", ").unwrap()))
        .map(|(start, (left, right))| (start.trim().to_owned(), left.replace(" (", ""), right.replace(")", "")));
    let mut map: HashMap<String, Node> = Default::default();
    for i in path {
        let map_ref = &mut map;
        map_ref.insert(i.0.clone(), Node {left: i.1, right: i.2});
    }
    let out = iterate(inputs, first, map.clone());
    out
}
fn iterate(inputs: Vec<bool>, first: String, map: HashMap<String, Node>) -> u64 {
    let mut map = map;
    let mut steps: usize = 0;
    let mut curr: String = first.clone();
    // dbg!(map.get(&first));
    let mut i = 0;
    while curr != "ZZZ" {
        steps += 1;
        if i < steps {
            i += 1;
        }
        if i == inputs.len() {
            i = 0;
        }
        let turn = inputs[i];

        if turn {
        }
        else {
            curr = (*map.get(&curr).unwrap().left).parse().unwrap();
        }
    }
    0
}