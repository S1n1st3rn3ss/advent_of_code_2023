#[allow(clippy::all)]
use std::fs;
use criterion::{Criterion, criterion_group, criterion_main, black_box};
use advent_of_code_2023::{day1, day5, day7, day8};
pub fn day1_benchmark(c: &mut Criterion) {
    let mut string: String = fs::read_to_string("/home/chiffa/CLionProjects/advent_of_code_2023/benches/test_inputs/day1.txt").expect("");
    c.bench_function("day 1 part 1", |b| b.iter(||day1::part1::run(&string)));
    c.bench_function("day 1 part 2", |b| b.iter(||day1::part2::run(&string)));
}
// pub fn day2_benchmark(c: &mut Criterion) {
//     let mut string: String = fs::read_to_string("/home/chiffa/CLionProjects/advent_of_code_2023/benches/test_inputs/day1.txt").expect("");
//     c.bench_function("day 2 part 1", |b| b.iter(||day1::part1::run(&string)));
//     c.bench_function("day 2 part 2", |b| b.iter(||day1::part2::run(&string)));
// }
// pub fn day3_benchmark(c: &mut Criterion) {
//     let mut string: String = fs::read_to_string("/home/chiffa/CLionProjects/advent_of_code_2023/benches/test_inputs/day1.txt").expect("");
//     c.bench_function("day 2 part 1", |b| b.iter(||day1::part1::run(&string)));
//     c.bench_function("day 2 part 2", |b| b.iter(||day1::part2::run(&string)));
// }
// pub fn day4_benchmark(c: &mut Criterion) {
//     let mut string: String = fs::read_to_string("/home/chiffa/CLionProjects/advent_of_code_2023/benches/test_inputs/day1.txt").expect("");
//     c.bench_function("day 2 part 1", |b| b.iter(||day1::part1::run(&string)));
//     c.bench_function("day 2 part 2", |b| b.iter(||day1::part2::run(&string)));
// }
pub fn day5_benchmark(c: &mut Criterion) {
    let mut string: String = fs::read_to_string("/home/chiffa/CLionProjects/advent_of_code_2023/benches/test_inputs/day5.txt").expect("");
    c.bench_function("day 5 part 1", |b| b.iter(||day5::part1::run(&string)));
    // Takes over 2h in initial form
    // c.bench_function("day 5 part 2", |b| b.iter(||day5::part2::run(&string)));
}
// pub fn day6_benchmark(c: &mut Criterion) {
//     let mut string: String = fs::read_to_string("/home/chiffa/CLionProjects/advent_of_code_2023/benches/test_inputs/day1.txt").expect("");
//     c.bench_function("day 2 part 1", |b| b.iter(||day1::part1::run(&string)));
//     c.bench_function("day 2 part 2", |b| b.iter(||day1::part2::run(&string)));
// }
pub fn day7_benchmark(c: &mut Criterion) {
    let mut string: String = fs::read_to_string("/home/chiffa/CLionProjects/advent_of_code_2023/benches/test_inputs/day7.txt").expect("");
    c.bench_function("day 7 part 1", |b| b.iter(||day7::part1::run(&string)));
    c.bench_function("day 7 part 2", |b| b.iter(||day7::part2::run(&string)));
}

pub fn day8_benchmark(c: &mut Criterion) {
    let mut string: String = fs::read_to_string("/home/chiffa/CLionProjects/advent_of_code_2023/benches/test_inputs/day8.txt").expect("");
    c.bench_function("day 8 part 1", |b| b.iter(||day8::part1::run(&string)));
    c.bench_function("day 8 part 2", |b| b.iter(||day8::part2::run(&string)));
}

criterion_group!(benches, day1_benchmark, day5_benchmark, day7_benchmark, day8_benchmark);
criterion_main!(benches);