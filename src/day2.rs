use std::cmp::max;
use std::ops::Index;
use regex::Regex;
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
pub fn parse_game(input: Vec<String>) -> Vec<Game> {
    // trim "Game n: " from each string
    let mut games: Vec<Game> = Default::default();
    for i in input.clone().iter().enumerate()  {
        let mut game: Game = Game {
            id: (i.0 + 1) as i32,
            group: vec![],
            min_group: Default::default(),
            possible: true,
        };
        // println!("{:?}", game);
        let trimmed = i.1.trim_start_matches("Game ").trim_start_matches(|c:char| c.is_digit(10) || c == ':');
        let trimmed = trimmed.trim();
        // split games into rounds
        let trem: Vec<Vec<&str>> = trimmed
            .split("; ")
            .map(|c| c.split(", ").collect::<Vec<&str>>())
            .collect();
        for x in trem {
            let mut gr = Group {
                red: 0,
                green: 0,
                blue: 0
            };
            for y in x {
                if y.ends_with(" red") {
                    gr.red = y.trim_end_matches(" red").parse::<i32>().unwrap();
                }
                if y.ends_with(" green") {
                    gr.green = y.trim_end_matches(" green").parse::<i32>().unwrap();
                }
                if y.ends_with(" blue") {
                    gr.blue = y.trim_end_matches(" blue").parse::<i32>().unwrap();
                }
            }

            game.group.push(gr);
        }
        games.push(game);
    }
    games
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