mod aoc;
mod position;

use std::fs;
use std::time::{Instant, Duration};

const MICROSEC_ITERATIONS: u16 = 2000;
const MILLISEC_ITERATIONS: u16 = 300;
const CENTISEC_ITERATIONS: u16 = 40;
const DECISEC_ITERATIONS:  u16 = 10;
const SEC_ITERATIONS:      u16 = 3;

#[allow(non_snake_case)]
fn main() {
    println!("{}", run_bench("day01.txt", &aoc::d01_part1, "d01p1", MICROSEC_ITERATIONS));
    println!("{}", run_bench("day01.txt", &aoc::d01_part2, "d01p2", MICROSEC_ITERATIONS));
    println!("{}", run_bench("day02.txt", &aoc::d02_part1, "d02p1", MICROSEC_ITERATIONS));
    println!("{}", run_bench("day02.txt", &aoc::d02_part2, "d02p2", MICROSEC_ITERATIONS));
    println!("{}", run_bench("day03.txt", &aoc::d03_part1, "d03p1", MICROSEC_ITERATIONS));
    println!("{}", run_bench("day03.txt", &aoc::d03_part2, "d03p2", MICROSEC_ITERATIONS));
    println!("{}", run_bench("day04.txt", &aoc::d04_part1, "d04p1", MILLISEC_ITERATIONS));
    println!("{}", run_bench("day04.txt", &aoc::d04_part2, "d04p2", MILLISEC_ITERATIONS));
    println!("{}", run_bench("day05.txt", &aoc::d05_part1, "d05p1", MICROSEC_ITERATIONS));
    println!("{}", run_bench("day05.txt", &aoc::d05_part2, "d05p2", MICROSEC_ITERATIONS));
    println!("{}", run_bench("day06.txt", &aoc::d06_part1, "d06p1", MICROSEC_ITERATIONS));
    println!("{}", run_bench("day06.txt", &aoc::d06_part2, "d06p2", SEC_ITERATIONS));
    println!("{}", run_bench("day07.txt", &aoc::d07_part1, "d07p1", MILLISEC_ITERATIONS));
    println!("{}", run_bench("day07.txt", &aoc::d07_part2, "d07p2", SEC_ITERATIONS));
    println!("{}", run_bench("day08.txt", &aoc::d08_part1, "d08p1", MILLISEC_ITERATIONS));
    println!("{}", run_bench("day08.txt", &aoc::d08_part2, "d08p2", MILLISEC_ITERATIONS));
    println!("{}", run_bench("day09.txt", &aoc::d09_part1, "d09p1", MICROSEC_ITERATIONS));
    println!("{}", run_bench("day09.txt", &aoc::d09_part2, "d09p2", CENTISEC_ITERATIONS));
    println!("{}", run_bench("day10.txt", &aoc::d10_part1, "d10p1", DECISEC_ITERATIONS));
    println!("{}", run_bench("day10.txt", &aoc::d10_part2, "d10p2", MICROSEC_ITERATIONS));
    println!("{}", run_bench("day11.txt", &aoc::d11_part1, "d11p1", CENTISEC_ITERATIONS));
    println!("{}", run_bench("day11.txt", &aoc::d11_part2, "d11p2", MILLISEC_ITERATIONS));
    println!("{}", run_bench("day12.txt", &aoc::d12_part1, "d12p1", CENTISEC_ITERATIONS));
    println!("{}", run_bench("day12.txt", &aoc::d12_part2, "d12p2", CENTISEC_ITERATIONS));
    println!("{}", run_bench("day13.txt", &aoc::d13_part1, "d13p1", MICROSEC_ITERATIONS));
    println!("{}", run_bench("day13.txt", &aoc::d13_part2, "d13p2", MICROSEC_ITERATIONS));
    println!("{}", run_bench("day14.txt", &aoc::d14_part1, "d14p1", MICROSEC_ITERATIONS));
    println!("{}", run_bench("day14.txt", &aoc::d14_part2, "d14p2", CENTISEC_ITERATIONS));
    println!("{}", run_bench("day15.txt", &aoc::d15_part1, "d15p1", MILLISEC_ITERATIONS));
    println!("{}", run_bench("day15.txt", &aoc::d15_part2, "d15p2", CENTISEC_ITERATIONS));
    println!("{}", run_bench("day16.txt", &aoc::d16_part1, "d16p1", MILLISEC_ITERATIONS));
    println!("{}", run_bench("day16.txt", &aoc::d16_part2, "d16p2", CENTISEC_ITERATIONS));
}


fn run_bench(input_file_name: &str, function: &dyn Fn(String) -> String, function_name: &str, iterations: u16) -> String {
    let mut times: Vec<Duration> = Vec::new();
    let mut result: String = "".to_string();

    for _ in 0..iterations {
        let now: Instant = Instant::now();
        let contents: String = fs::read_to_string(input_file_name).expect(format!("Should have been able to read: {}", input_file_name).as_str());
        result = function(contents.clone());
        times.push(now.elapsed())
    }

    // get the average
    let mut mean_time: Duration = Duration::new(0, 0);
    for time in times.clone() {
        mean_time += time;
    }
    mean_time /= times.len() as u32;

    return format!("{}: {:>8.2?} (answer: {})", function_name, mean_time, result);
}