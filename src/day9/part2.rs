use std::str::Chars;
use hashbrown::HashMap;
pub fn run(input: &str) -> i32 {
    let mut input = parse(input);
    input
}
fn parse(input: &str) -> i32 {
    let lines: Vec<Vec<i32>> = input
        .trim()
        .lines()
        .map(|x| x.split_whitespace().collect::<Vec<&str>>())
        .map(|x| x.iter().map(|x| x.parse::<i32>().unwrap()).collect())
        .collect();
    let mut sum = 0;
    for i in lines {
        sum += create_history(i);
    }
    sum
}
fn create_history(input: Vec<i32>) -> i32 {
    let mut v = vec![vec![]];
    v[0] = input;
    let l = v[0].len() + 1;
    let mut i = 1;
    while v[v.len() - 1] != vec![0; v[v.len() - 1].len()] {
        v.push(vec![]);
        for j in 0..v[i - 1].len() - 1{
            let value = v[i - 1][j + 1] - v[i - 1][j];
            v[i].push(value)
        }
        i += 1;
    }
    i -= 1;
    v[i].insert(0,0);
    while v[0].len() != l {
        let value = v[i - 1][0] - v[i][0];
        v[i - 1].insert(0, value);
        i -= 1;
    }
    v[0][0]
}