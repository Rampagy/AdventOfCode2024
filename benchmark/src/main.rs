mod aoc;
mod position;

use std::env;
use std::fs;
//use std::thread;
//use num_cpus;
use std::time::{Instant, Duration};
//use std::sync::{Arc, Mutex};


const XLOW_MICROSEC_ITERATIONS: u32 = 200001; // ~1 second @ 50 microseconds per iteration
const LOW_MICROSEC_ITERATIONS: u32 = 20001; // ~1 second @ 50 microseconds per iteration
const MICROSEC_ITERATIONS: u32 = 2001; // ~1 second @ 500 microseconds per iteration
const MILLISEC_ITERATIONS: u32 = 201;// ~1 second @ 5 milliseconds per iteration
const CENTISEC_ITERATIONS: u32 = 21; // ~1 second @ 5 centiseconds per iteration
const DECISEC_ITERATIONS:  u32 = 6; // ~3 second @ 500 deciseconds per iteration
const SEC_ITERATIONS:      u32 = 1; // minimum is at least 3 iterations


#[allow(non_snake_case)]
fn main() {
    let args: Vec<String> = env::args().collect();

    let benchmarks: Vec<(&str, fn(String) -> String, &str, u32)> = vec![
        ("day01.txt", aoc::d01_part1 as fn(String) -> String, "d01p1", MICROSEC_ITERATIONS),
        ("day01.txt", aoc::d01_part2 as fn(String) -> String, "d01p2", MICROSEC_ITERATIONS),
        ("day02.txt", aoc::d02_part1 as fn(String) -> String, "d02p1", MICROSEC_ITERATIONS),
        ("day02.txt", aoc::d02_part2 as fn(String) -> String, "d02p2", MICROSEC_ITERATIONS),
        ("day03.txt", aoc::d03_part1 as fn(String) -> String, "d03p1", MICROSEC_ITERATIONS),
        ("day03.txt", aoc::d03_part2 as fn(String) -> String, "d03p2", LOW_MICROSEC_ITERATIONS),
        ("day04.txt", aoc::d04_part1 as fn(String) -> String, "d04p1", MILLISEC_ITERATIONS),
        ("day04.txt", aoc::d04_part2 as fn(String) -> String, "d04p2", MILLISEC_ITERATIONS),
        ("day05.txt", aoc::d05_part1 as fn(String) -> String, "d05p1", MICROSEC_ITERATIONS),
        ("day05.txt", aoc::d05_part2 as fn(String) -> String, "d05p2", MICROSEC_ITERATIONS),
        ("day06.txt", aoc::d06_part1 as fn(String) -> String, "d06p1", MICROSEC_ITERATIONS),
        ("day06.txt", aoc::d06_part2 as fn(String) -> String, "d06p2", SEC_ITERATIONS),
        ("day07.txt", aoc::d07_part1 as fn(String) -> String, "d07p1", MILLISEC_ITERATIONS),
        ("day07.txt", aoc::d07_part2 as fn(String) -> String, "d07p2", SEC_ITERATIONS),
        ("day08.txt", aoc::d08_part1 as fn(String) -> String, "d08p1", MILLISEC_ITERATIONS),
        ("day08.txt", aoc::d08_part2 as fn(String) -> String, "d08p2", MILLISEC_ITERATIONS),
        ("day09.txt", aoc::d09_part1 as fn(String) -> String, "d09p1", MICROSEC_ITERATIONS),
        ("day09.txt", aoc::d09_part2 as fn(String) -> String, "d09p2", CENTISEC_ITERATIONS),
        ("day10.txt", aoc::d10_part1 as fn(String) -> String, "d10p1", DECISEC_ITERATIONS),
        ("day10.txt", aoc::d10_part2 as fn(String) -> String, "d10p2", LOW_MICROSEC_ITERATIONS),
        ("day11.txt", aoc::d11_part1 as fn(String) -> String, "d11p1", CENTISEC_ITERATIONS),
        ("day11.txt", aoc::d11_part2 as fn(String) -> String, "d11p2", MILLISEC_ITERATIONS),
        ("day12.txt", aoc::d12_part1 as fn(String) -> String, "d12p1", CENTISEC_ITERATIONS),
        ("day12.txt", aoc::d12_part2 as fn(String) -> String, "d12p2", CENTISEC_ITERATIONS),
        ("day13.txt", aoc::d13_part1 as fn(String) -> String, "d13p1", MICROSEC_ITERATIONS),
        ("day13.txt", aoc::d13_part2 as fn(String) -> String, "d13p2", LOW_MICROSEC_ITERATIONS),
        ("day14.txt", aoc::d14_part1 as fn(String) -> String, "d14p1", LOW_MICROSEC_ITERATIONS),
        ("day14.txt", aoc::d14_part2 as fn(String) -> String, "d14p2", CENTISEC_ITERATIONS),
        ("day15.txt", aoc::d15_part1 as fn(String) -> String, "d15p1", MILLISEC_ITERATIONS),
        ("day15.txt", aoc::d15_part2 as fn(String) -> String, "d15p2", CENTISEC_ITERATIONS),
        ("day16.txt", aoc::d16_part1 as fn(String) -> String, "d16p1", MILLISEC_ITERATIONS),
        ("day16.txt", aoc::d16_part2 as fn(String) -> String, "d16p2", CENTISEC_ITERATIONS),
        ("day17.txt", aoc::d17_part1 as fn(String) -> String, "d17p1", XLOW_MICROSEC_ITERATIONS),
        ("day17.txt", aoc::d17_part2 as fn(String) -> String, "d17p2", LOW_MICROSEC_ITERATIONS),
        ("day18.txt", aoc::d18_part1 as fn(String) -> String, "d18p1", MILLISEC_ITERATIONS),
        ("day18.txt", aoc::d18_part2 as fn(String) -> String, "d18p2", DECISEC_ITERATIONS),
        ("day19.txt", aoc::d19_part1 as fn(String) -> String, "d19p1", CENTISEC_ITERATIONS),
        ("day19.txt", aoc::d19_part2 as fn(String) -> String, "d19p2", CENTISEC_ITERATIONS),
        ("day20.txt", aoc::d20_part1 as fn(String) -> String, "d20p1", CENTISEC_ITERATIONS),
        ("day20.txt", aoc::d20_part2 as fn(String) -> String, "d20p2", CENTISEC_ITERATIONS),
        ("day21.txt", aoc::d21_part1 as fn(String) -> String, "d21p1", LOW_MICROSEC_ITERATIONS),
        ("day21.txt", aoc::d21_part2 as fn(String) -> String, "d21p2", MICROSEC_ITERATIONS),
        ("day22.txt", aoc::d22_part1 as fn(String) -> String, "d22p1", MILLISEC_ITERATIONS),
        ("day22.txt", aoc::d22_part2 as fn(String) -> String, "d22p2", SEC_ITERATIONS),
        ("day23.txt", aoc::d23_part1 as fn(String) -> String, "d23p1", SEC_ITERATIONS),
        ("day23.txt", aoc::d23_part2 as fn(String) -> String, "d23p2", DECISEC_ITERATIONS),
        ("day24.txt", aoc::d24_part1 as fn(String) -> String, "d24p1", MICROSEC_ITERATIONS),
        ("day24.txt", aoc::d24_part2 as fn(String) -> String, "d24p2", MILLISEC_ITERATIONS),
        ("day25.txt", aoc::d25_part1 as fn(String) -> String, "d25p1", MICROSEC_ITERATIONS),
    ];

    let arg_contains_multi: bool = args.contains(&"--multi".to_string());
    let arg_contains_both: bool = args.contains(&"--both".to_string());

    if arg_contains_multi || arg_contains_both {
        /*
        let (sender, receiver) = channel::unbounded();
        let results = Arc::new(Mutex::new(Vec::new()));

        for benchmark in benchmarks {
            sender.send(benchmark).unwrap();
        }

        let mut children = Vec::new();
        for _ in 0..num_cpus::get() {
            let receiver = receiver.clone();
            let results = Arc::clone(&results);
            children.push(thread::spawn(move || {
                while let Ok((file, func, label, iterations)) = receiver.recv() {
                    let result = run_multi_bench(file, func, label, iterations);
                    results.lock().unwrap().push(result);
                }
            }));
        }

        for child in children {
            child.join().unwrap();
        }

        let mut results = Arc::try_unwrap(results).expect("Mutex still has multiple owners").into_inner().expect("Mutex cannot be locked");
        results.sort();
        for result in results {
            println!("{}", result);
        }
        */
        println!("TODO: multithreaded benchmark");
    }

    if arg_contains_both {
        // print some blank spaces to seperate the results
        println!("\n");
    }

    if arg_contains_both || !arg_contains_multi {
        let now: Instant = Instant::now();
        println!("{:^5} | {:^9} | {:^9} | {:^8}", "", "Iteration", "Total", "");
        println!("{:^5} | {:^9} | {:^9} | {:^8}", "Part", "time", "time", "Answer");
        println!("---------------------------------------------");

        for (input_file, function, part, iterations) in benchmarks {
            println!("{}", run_bench(input_file, &function, part, iterations));
        }

        println!("----------------------------------------------------------------");
        println!("{:>29.2?}", now.elapsed());
    }
}


fn run_bench(input_file_name: &str, function: &dyn Fn(String) -> String, function_name: &str, iterations: u32) -> String {
    let mut times: Vec<Duration> = Vec::new();
    let mut result: String = "".to_string();

    if let Ok(contents) = fs::read_to_string(input_file_name) {
        for _ in 0..iterations {
            let now: Instant = Instant::now();
            result = function(contents.clone());
            times.push(now.elapsed())
        }

        // get the average
        let mut mean_time: Duration = Duration::new(0, 0);
        for time in times.clone() {
            mean_time += time;
        }
        let total_time: Duration = mean_time;
        mean_time /= times.len() as u32;

        return format!("{} | {:>9.2?} | {:>9.2?} | {}", function_name, mean_time, total_time, result);
    } else {
        // did not read text file properly
        return format!("{} | Unable to read the file {}", function_name, input_file_name);
    }
}