use std::fs;
use std::time::Instant;
use std::collections::HashMap;


#[allow(non_snake_case)]
fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");
    let mut now: Instant;
    let mut elapsed: std::time::Duration;

    now = Instant::now();
    let part1: u64 = part1(contents.clone());
    elapsed = now.elapsed();

    println!("part 1: {} ({:.2?})", part1, elapsed);

    now = Instant::now();
    let part2: u64 = part2(contents.clone());
    elapsed = now.elapsed();

    println!("part 2: {} ({:.2?})", part2, elapsed);
}

fn depth_first_search(options: &Vec<String>, pattern: String, cache: &mut HashMap<String, u64>) -> u64 {
    if let Some(count) = cache.get(&pattern) {
        return *count;
    }

    let mut combinations: u64 = 0;
    if pattern.is_empty() {
        combinations = 1;
    } else {
        for option in options {
            if let Some(remaining_pattern) = pattern.strip_prefix(option) {
                // there's patterns left to match
                combinations += depth_first_search(options, remaining_pattern.to_string(), cache);
            }
        }
    }

    // add to cache
    cache.insert(pattern, combinations);
    return combinations;
}


#[allow(non_snake_case)]
fn part1(contents: String) -> u64 {
    let mut towels: Vec<String> = Vec::new();
    let mut capture_options: bool = true;
    let mut valid_patterns: u64 = 0;
    let mut cache: HashMap<String, u64> = HashMap::new();

    for (_line_num, line) in contents.lines().enumerate() {
        if line == "" {
            capture_options = false;
        } else {
            if capture_options {
                towels.append(&mut line.split(", ").map(|x| x.to_string()).collect::<Vec<String>>());
            } else {
                // this is where each towel needs to be reproduced
                if depth_first_search(&(towels.clone()), line.to_string(), &mut cache) > 0 {
                    valid_patterns += 1;
                }
            }
        }
    }

    return valid_patterns;
}



#[allow(non_snake_case)]
fn part2(contents: String) -> u64 {
    let mut towels: Vec<String> = Vec::new();
    let mut capture_options: bool = true;
    let mut valid_patterns: u64 = 0;
    let mut cache: HashMap<String, u64> = HashMap::new();

    for (_line_num, line) in contents.lines().enumerate() {
        if line == "" {
            capture_options = false;
        } else {
            if capture_options {
                towels.append(&mut line.split(", ").map(|x| x.to_string()).collect::<Vec<String>>());
            } else {
                // this is where each towel needs to be reproduced
                valid_patterns += depth_first_search(&(towels.clone()), line.to_string(), &mut cache);
            }
        }
    }

    return valid_patterns;
}



#[cfg(test)] #[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test_part1a() {
        let contents: String = fs::read_to_string("src/test1a.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 6);
    }

    #[test]
    fn test_part2a() {
        let contents: String = fs::read_to_string("src/test2a.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), 16);
    }
}