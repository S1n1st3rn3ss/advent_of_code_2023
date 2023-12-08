use std::ops::BitOr;
use num_integer::{Integer, lcm};
use hashbrown::HashMap;
pub fn run(input: &str) -> u64 {
    let mut input = parse(input);
    input
}
#[derive(Debug, Clone)]
struct Direction<'a> {
    left: &'a str,
    right: &'a str
}
fn parse(input: &str) -> u64 {
    let lines: Vec<String> = input
        .trim()
        .lines()
        .map(|x| x.to_owned())
        .collect();
    let inputs: Vec<bool> = lines[0].chars().map(|x| x == 'R').collect();
    let path: Vec<_> = lines.iter().skip(2)
        .map(|x| x.split_once('=').unwrap())
        .map(|(start, end)| (start, end.split_once(", ").unwrap()))
        .collect();
    let path: Vec<(&str, (&str, &str))> = path.iter()
        .map(|(start, (left, right))|
            (start.trim(), (left.strip_prefix(" (").unwrap(), right.strip_suffix(")").unwrap())))
        .collect();
    let mut map: HashMap<&str, Direction> = Default::default();
    for i in path {
        let map_ref = &mut map;
        map_ref.insert(i.0, Direction {left: i.1.0, right: i.1.1});
    }
    iterate(inputs, &map)
}
#[derive(Debug, Clone)]
struct Node<'a> {
    curr: &'a str,
    count: i32,
    dirs: &'a Direction<'a>,
    locked: bool
}
fn iterate(inputs: Vec<bool>, map: &HashMap<&str, Direction>) -> u64 {
    let mut map = map;
    let mut steps: usize = 0;
    let mut a_map: HashMap<&str, Node> = Default::default();
    for (k, v) in map {
        if k.as_bytes()[2] == b'A' {
            let node = Node {
                curr: k,
                count: 1,
                locked: false,
                dirs: map.get(k).unwrap(),
            };
            a_map.insert(k, node);
        }
    }
    let mut all_locked = false;
    while !all_locked {
        let turn = inputs[steps % inputs.len()];
        let re = &mut a_map;
        for (idx, node) in re {
            let curr = if turn {node.dirs.right} else {node.dirs.left};
            let lock = (&curr.chars().last().unwrap() == &'Z').bitor(node.locked);
            *node = Node {
                curr, count: node.count + if lock { 0 } else { 1 }, dirs: map.get(curr).unwrap(), locked: lock
            };
        }
        steps += 1;
        let mut locks = 0;
        for i in &a_map {
            if i.1.locked {
                locks += 1;
            }
        }
        all_locked = locks == a_map.len();
    }
    let mut lcm: u64 = 1;
    for (_, node) in a_map {
        lcm = lcm.lcm(&(node.count as u64));
    }
    lcm
}
