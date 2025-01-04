mod aoc;

use std::fs;
use std::time::{Instant, Duration};

const ITERATIONS: u16 = 10;

#[allow(non_snake_case)]
fn main() {
    println!("{}", run_bench("day01.txt", &aoc::d01_part1, "d01p1"));
    println!("{}", run_bench("day01.txt", &aoc::d01_part2, "d01p2"));

    println!("{}", run_bench("day02.txt", &aoc::d02_part1, "d02p1"));
    println!("{}", run_bench("day02.txt", &aoc::d02_part2, "d02p2"));

    println!("{}", run_bench("day03.txt", &aoc::d03_part1, "d03p1"));
    println!("{}", run_bench("day03.txt", &aoc::d03_part2, "d03p2"));

    println!("{}", run_bench("day04.txt", &aoc::d04_part1, "d04p1"));
    println!("{}", run_bench("day04.txt", &aoc::d04_part2, "d04p2"));

    println!("{}", run_bench("day05.txt", &aoc::d05_part1, "d05p1"));
    println!("{}", run_bench("day05.txt", &aoc::d05_part2, "d05p2"));
}

fn run_bench(input_file_name: &str, function: &dyn Fn(String) -> String, function_name: &str) -> String {
    let mut times: Vec<Duration> = Vec::new();
    let mut result: String = "".to_string();

    for _ in 0..ITERATIONS {
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