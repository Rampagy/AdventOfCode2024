use std::fs;
use std::time::Instant;
use std::collections::HashMap;


#[allow(non_snake_case)]
fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");
    let mut now: Instant;
    let mut elapsed: std::time::Duration;

    now = Instant::now();
    let part1: u64 = part1(contents.clone(), 25);
    elapsed = now.elapsed();

    println!("part 1: {} ({:.2?})", part1, elapsed);

    now = Instant::now();
    let part2: u64 = part2(contents.clone(), 75);
    elapsed = now.elapsed();

    println!("part 2: {} ({:.2?})", part2, elapsed);
}


#[allow(non_snake_case)]
fn part1(contents: String, blinks: u64) -> u64 {
    let mut arrangement: Vec<u64> = Vec::new();

    for (_line_num, line) in contents.lines().enumerate() {
        arrangement = line.split_ascii_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
    }

    for _ in 0..blinks {
        let mut new_arrangement: Vec<u64> = Vec::new();
        for j in 0..arrangement.len() {
            if arrangement[j] == 0 {
                new_arrangement.push(1);
            } else if arrangement[j].to_string().len() & 0x01 == 0x00 {
                // even number of digits
                let num_as_str: String = arrangement[j].to_string();
                let num_digits: usize = num_as_str.len() >> 1;
                let left: u64 = num_as_str[..num_digits].parse::<u64>().unwrap();
                let right: u64 = num_as_str[num_digits..].parse::<u64>().unwrap();
                new_arrangement.push(left);
                new_arrangement.push(right);
            } else {
                new_arrangement.push(arrangement[j] * 2024);
            }
        }

        arrangement = new_arrangement;
    }

    return arrangement.len() as u64;
}


#[allow(non_snake_case)]
fn part2(contents: String, blinks: u64) -> u64 {
    let mut answer: u64 = 0;
    let mut arrangement: HashMap<u64, u64> = HashMap::new();

    for (_line_num, line) in contents.lines().enumerate() {
        for num in line.split_ascii_whitespace() {
            let digit: u64 = num.parse::<u64>().unwrap();

            match arrangement.get_mut(&digit) {
                Some(x) => {
                    *x += 1;
                } None => {
                    arrangement.insert(digit, 1);
                }
            }
        }
    }

    for _ in 0..blinks {
        let mut new_arrangement: HashMap<u64, u64> = HashMap::new();
        for (digit, count) in arrangement.iter() {
            if *digit == 0 {
                // CONVERT to a 1
                let new_digit: u64 = 1;
                match new_arrangement.get_mut(&new_digit) {
                    Some(x) => {
                        // increment the current count, if the digit is already in the hashmap
                        *x += *count;
                    } None => {
                        // if not in the hashmap, add it with the current count
                        new_arrangement.insert(new_digit, *count);
                    }
                }
            } else if digit.to_string().len() & 0x01 == 0x00 {
                // even number of digits
                let num_as_str: String = digit.to_string();
                let num_digits: usize = num_as_str.len() >> 1;
                let left: u64 = num_as_str[..num_digits].parse::<u64>().unwrap();
                let right: u64 = num_as_str[num_digits..].parse::<u64>().unwrap();

                for d in [left, right] {
                    match new_arrangement.get_mut(&d) {
                        Some(x) => {
                            // increment the current count, if the digit is already in the hashmap
                            *x += *count;
                        } None => {
                            // if not in the hashmap, add it with the current count
                            new_arrangement.insert(d, *count);
                        }
                    }
                }
            } else {
                // mutliply by 2024
                let new_digit: u64 = digit * 2024;
                match new_arrangement.get_mut(&new_digit) {
                    Some(x) => {
                        // increment the current count, if the digit is already in the hashmap
                        *x += *count;
                    } None => {
                        // if not in the hashmap, add it with the current count
                        new_arrangement.insert(new_digit, *count);
                    }
                }
            }
        }

        // overwrite the old arrangement with the newly constructed one
        arrangement = new_arrangement;
    }

    for (_digit, count) in arrangement.iter() {
        answer += count;
    }

    return answer;
}


#[cfg(test)] #[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test_part1a() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone(), 6), 22);
    }

    #[test]
    fn test_part1b() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone(), 25), 55312);
    }

    #[test]
    fn test_part2a() {
        let contents: String = fs::read_to_string("src/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone(), 6), 22);
    }

    #[test]
    fn test_part2b() {
        let contents: String = fs::read_to_string("src/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone(), 25), 55312);
    }
}