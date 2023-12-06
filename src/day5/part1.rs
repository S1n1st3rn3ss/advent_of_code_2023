use std::collections::HashMap;
use std::usize;

#[derive(Debug)]
struct Map {
    destination_start: u64,
    source_start: u64,
    range: u64,
}
#[derive(Debug)]
struct Maps {
    seeds: Vec<u64>,
    seed_soil: HashMap<u64, u64>,
    soil_fertilizer: HashMap<u64, u64>,
    fertilizer_water: HashMap<u64, u64>,
    water_light: HashMap<u64, u64>,
    light_temperature: HashMap<u64, u64>,
    temperature_humidity: HashMap<u64, u64>,
    humidity_location: HashMap<u64, u64>,
}
pub fn run(input: &str) -> u64 {
    parse(input);
    let strings: Vec<_> = parse(input);
    let seeds: Vec<u64> = strings[0][0].split(" ").skip(1).map(|x| x.parse::<u64>().unwrap()).collect();
    let seed_soil: Vec<Map> = get_maps(strings[1].clone());
    let soil_fertilizer: Vec<Map> = get_maps(strings[2].clone());
    let fertilizer_water: Vec<Map> = get_maps(strings[3].clone());
    let water_light: Vec<Map> = get_maps(strings[4].clone());
    let light_temperature: Vec<Map> = get_maps(strings[5].clone());
    let temperature_humidity: Vec<Map> = get_maps(strings[6].clone());
    let humidity_location: Vec<Map> = get_maps(strings[7].clone());
    let i = get_output(seeds, seed_soil);
    let i = get_output(i, soil_fertilizer);
    let i = get_output(i, fertilizer_water);
    let i = get_output(i, water_light);
    let i = get_output(i, light_temperature);
    let i = get_output(i, temperature_humidity);
    let i = get_output(i, humidity_location);
    let mut min = u64::MAX;
    for j in i {
        min = min.min(j);
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
    //     // .map(|x| x.to_owned())
        .collect();
    parsed
}
fn get_maps(input: Vec<String>) -> Vec<Map> {
    let maps: Vec<Map> = input
        .iter().skip(1)
        .map(|x|
            x.split_whitespace()
                .map(|x|
                    x.parse::<u64>()
                        .unwrap())
                .collect())
        .map(|x: Vec<u64>| Map { destination_start: x[0], source_start: x[1], range: x[2]})
        .collect();
    maps
}
fn get_output(seeds: Vec<u64>, maps: Vec<Map>) -> Vec<u64> {
    let mut seeds_output: Vec<u64> = vec![];
    for seed in seeds {
        let mut i = seed.clone();
        for map in &maps {
            let src_dest = map.destination_start  as i64 - map.source_start as i64;
            if seed > map.source_start && seed < (map.source_start + map.range) {
                let tmp = (seed as i64 + src_dest) as u64;
                i = tmp;
                break;
            } else {
                continue;
            }
        }
        seeds_output.push(i);
    }
    seeds_output.sort();
    seeds_output.dedup();
    seeds_output
}