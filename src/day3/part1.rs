use std::cmp::{max, min};
use std::collections::HashMap;
use std::hash::Hash;

pub fn parse_field(input: String) -> Vec<Vec<char>> {
    // create chat matrix
    // allows for simpler traversal
    let mut rows: Vec<Vec<char>> = vec![];
    for i in input.lines() {
        let mut column: Vec<char> = vec![];
        for j in i.chars() {
            column.push(j);
        }
        rows.push(column);
    }
    rows
}
pub fn find_symbols(input: Vec<Vec<char>>) -> (Vec<Vec<char>>, Vec<(usize, usize)>) {
    let mut coords: Vec<(usize, usize)> = vec![];
    for i in input.iter().enumerate() {
        for j in i.1.iter().enumerate() {
            if (j.1 != &'.') && (!j.1.is_digit(10)) {
                coords.push((i.0, j.0));
            }
        }
    }
    (input, coords)
}
#[derive(Debug, Copy, Clone)]
struct Number {
    number: i32,
    // left-most X
    x: usize,
    y: usize,
}
pub fn find_numbers(input: Vec<Vec<char>>) {
    let mut numbers: Vec<Number> = vec![];
    for (y, line) in input.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if char.is_ascii_digit() {
                let tmp = Number {
                    number: char.to_digit(10).unwrap() as i32,
                    x,
                    y,
                };
                numbers.push(tmp)
            }
        }
    }
    let mut i: usize = 0;
    let mut num_fix: Vec<Number> = vec![];
    while i < numbers.len() {
        let mut num = Number {
            number: numbers[i].number,
            x: numbers[i].x,
            y: numbers[i].y,
        };
        i += 1;
        if num.x + 1 == numbers[min(i, numbers.len() - 1)].x {
            num.number = (num.number * 10) + numbers[i].number;
            i += 1;
            if num.x + 2 == numbers[min(i, numbers.len() - 1)].x {
                i = min(i, numbers.len() - 1);
                num.number = (num.number * 10) + numbers[i].number;
                i += 1;
            }
        }

        println!("{:?}", num.number);
        num_fix.push(num);
    }
    println!("-------------------------");
    let mut sum: i32 = 0;
    let mut hmap: HashMap<(usize, usize), Vec<i32>> = Default::default();
    for i in &num_fix {
        let g = Gear {
            coords: (0, 0),
            parts: vec![]
        };
        let left_bound = i.x.saturating_sub(1);
        let right_bound = min(i.x + i.number.to_string().len(), input[0].len() - 1);
        let top_bound = i.y.saturating_sub(1);
        let bottom_bound = min(i.y + 1, input.len() - 1);
        for x in left_bound..=right_bound {
            for y in top_bound..=bottom_bound {
                let curr = input[y][x];
                if (!curr.is_ascii_digit()) && (curr != '.') {
                    sum += i.number;
                    println!("{}, {}:{}", i.number, i.x, i.y);
                    if curr == '*' {
                        if !hmap.contains_key(&(x, y)) {
                            hmap.insert((x,y), vec![i.number]);
                        } else {
                            let it: &mut Vec<i32> = hmap.get_mut(&(x, y)).unwrap();
                            it.push(i.number);
                        }
                    }
                }
            }
        }
    }
    let mut sum = 0;
    for i in hmap.values() {
        if i.len() == 2 {
            sum += i[0] * i[1];
        }
    }
    println!("{sum}");
    // let mut gear_coords: Vec<Gear> = vec![];
    // for (y, line) in input.lines().enumerate() {
    //     for (x, sym) in line.chars().enumerate() {
    //         if sym == '*' {
    //             gear_coords.push(Gear {
    //                 coords: (x, y),
    //                 .. Gear::default()
    //             });
    //         }
    //     }
    // }
    println!("-------------------------");
    println!("{sum}")
}
#[derive(Default, Debug, Clone)]
struct Gear {
    coords: (usize, usize),
    parts: Vec<u32>,
}
pub fn find_gears(input: String) {



}