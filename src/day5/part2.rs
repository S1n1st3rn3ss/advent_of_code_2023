use std::collections::HashMap;
use std::ops::{Range, RangeInclusive};

#[derive(Debug)]
struct Map {
    destination_start: i64,
    source_start: i64,
    range: i64,
}
pub fn run(input: &str) -> i64 {
    parse(input);
    let strings: Vec<_> = parse(input);
    let seeds: Vec<i64> = strings[0][0].split(" ").skip(1).map(|x| x.parse::<i64>().unwrap()).collect();
    let mut seeds_ranges: Vec<Range<i64>> = vec![];
    let mut i = 0;
    while i < seeds.len() {
        seeds_ranges.push(seeds[i]..seeds[i] + seeds[i+1]);
        i += 2;
    }
    let seed_soil: Vec<Map> = get_maps(strings[1].clone());
    let soil_fertilizer: Vec<Map> = get_maps(strings[2].clone());
    let fertilizer_water: Vec<Map> = get_maps(strings[3].clone());
    let water_light: Vec<Map> = get_maps(strings[4].clone());
    let light_temperature: Vec<Map> = get_maps(strings[5].clone());
    let temperature_humidity: Vec<Map> = get_maps(strings[6].clone());
    let humidity_location: Vec<Map> = get_maps(strings[7].clone());
    let mut min = i64::MAX as i64;
    for range in seeds_ranges {
        for seed in range {
            // println!("{}", seed);
            let i = get_output(seed, &seed_soil);
            let i = get_output(i, &soil_fertilizer);
            let i = get_output(i, &fertilizer_water);
            let i = get_output(i, &water_light);
            let i = get_output(i, &light_temperature);
            let i = get_output(i, &temperature_humidity);
            let i = get_output(i, &humidity_location);
            min = min.min(i);
        }
    }
    min
}
fn parse(input: &str) -> Vec<Vec<String>> {
    let parsed: Vec<_> = input
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let parsed: Vec<_> = parsed
        .split(|x| x.is_empty())
        .map(|x| x.to_vec())
        .collect();
    parsed
}
fn get_maps(input: Vec<String>) -> Vec<Map> {
    let maps: Vec<Map> = input
        .iter().skip(1)
        .map(|x|
            x.split_whitespace()
                .map(|x|
                    x.parse::<i64>()
                        .unwrap())
                .collect())
        .map(|x: Vec<i64>| Map { destination_start: x[0], source_start: x[1], range: x[2]})
        .collect();
    maps
}
fn get_output(seed: i64, maps: &Vec<Map>) -> i64 {
    let mut i = seed.clone();
    for map in maps {
        let src_dest = map.destination_start - map.source_start;
        if seed >= map.source_start && seed < (map.source_start + map.range) {
            let tmp = (seed + src_dest);
            i = tmp;
            break;
        } else {
            continue;
        }
    }
    i
}