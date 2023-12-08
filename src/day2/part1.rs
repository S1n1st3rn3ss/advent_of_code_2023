use std::cmp::max;
use std::ops::Index;
use regex::Regex;

// TODO: Split into parts and bench
#[derive(Default, Debug)]
pub struct Group {
    red: i32,
    green: i32,
    blue: i32
}
#[derive(Default, Debug)]
pub struct Game {
    id: i32,
    group: Vec<Group>,
    min_group: Group,
    possible: bool
}
pub fn run(input: &str) -> i32 {
    let parsed: _ = input
        .lines()
        .map(|x|
            x.split_once(':')
                .unwrap()
                .1.trim())
        .map(|x|
            x.split(';')
                .map(|x| x.trim()).collect())
        .collect::<Vec<Vec<&str>>>();
    let mut games: Vec<Game> = Default::default();
    for (id, res) in parsed.iter().enumerate()  {
        let mut game: Game = Game {
            id: (id + 1) as i32,
            group: vec![],
            min_group: Default::default(),
            possible: true,
        };
        for x in res {
            let mut gr = Group {
                red: 0,
                green: 0,
                blue: 0
            };
            dbg!(x);
            if x.ends_with(" red") {
                gr.red = x.trim_end_matches(" red").parse::<i32>().unwrap();
            }
            if x.ends_with(" green") {
                gr.green = x.trim_end_matches(" green").parse::<i32>().unwrap();
            }
            if x.ends_with(" blue") {
                gr.blue = x.trim_end_matches(" blue").parse::<i32>().unwrap();
            }
            game.group.push(gr);
        }
        // games.push(game);
    }
    // check_possibility(games)
    0
}


pub fn check_possibility(games: Vec<Game>) -> i32 {
    let mut group_sum: i32 = 0;
    let mut pow_sum: i32 = 0;
    for mut i in games {
        let mut gr: Group = Group {
            red: 0,
            green: 0,
            blue: 0,
        };
        for j in i.group {
            gr.red = max(gr.red, j.red);
            gr.green = max(gr.green, j.green);
            gr.blue = max(gr.blue, j.blue);
            if (j.red > 12) || (j.green > 13) || (j.blue > 14) {
                i.possible = false;
            }
        }
        if i.possible {
            group_sum += i.id;
        }
        pow_sum += gr.red * gr.green * gr.blue;

    }
    println!("{}", pow_sum);
    group_sum
}