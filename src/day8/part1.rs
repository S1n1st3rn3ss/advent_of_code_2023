use std::str::Chars;
use hashbrown::HashMap;
pub fn run(input: &str) -> u64 {
    let mut input = parse(input);
    input
}
#[derive(Debug, Clone)]
struct Node<'a> {
    left: &'a str,
    right: &'a str
}
fn parse(input: &str) -> u64 {
    let lines: Vec<String> = input
        .trim()
        .lines()
        .map(|x| x.to_owned())
        .collect();
    // True = Right, False = Right
    let inputs: Vec<bool> = lines[0].chars().map(|x| x == 'R').collect();
    let path: Vec<_> = lines.iter().skip(2)
        .map(|x| x.split_once('=').unwrap())
        .map(|(start, end)| (start, end.split_once(", ").unwrap()))
        .collect();
    let path: Vec<(&str, (&str, &str))> = path.iter()
        .map(|(start, (left, right))|
            (start.trim(), (left.strip_prefix(" (").unwrap(), right.strip_suffix(")").unwrap())))
        .collect();
    let mut map: HashMap<&str, Node> = Default::default();
    for i in path {
        let map_ref = &mut map;
        map_ref.insert(i.0, Node {left: i.1.0, right: i.1.1});
    }
    let out = iterate(inputs, &map);
    out
}
fn iterate(inputs: Vec<bool>, map: &HashMap<&str, Node>) -> u64 {
    let mut map = map;
    let mut steps: usize = 0;
    let mut curr: &str = "AAA";
    while curr != "ZZZ" {
        let turn = inputs[steps % inputs.len()];
        if turn {
            curr = map.get(&curr).unwrap().right;
        }
        else {
            curr = map.get(&curr).unwrap().left;
        }
        steps += 1;
    }
    steps as u64
}