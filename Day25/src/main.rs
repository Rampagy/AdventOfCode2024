use std::fs;
use std::time::Instant;


#[allow(non_snake_case)]
fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");
    let now: Instant;
    let elapsed: std::time::Duration;

    now = Instant::now();
    let part1: u64 = part1(contents.clone());
    elapsed = now.elapsed();

    println!("part 1: {} ({:.2?})", part1, elapsed);
}



#[allow(non_snake_case)]
fn part1(contents: String) -> u64 {
    let mut lock_line: u64 = 0;
    let mut current_lock_key: [u8; 5] = [0; 5];

    let mut locks: Vec<[u8; 5]> = Vec::new();
    let mut keys: Vec<[u8; 5]> = Vec::new();

    let mut is_lock: bool = false;
    for line in contents.lines() {
        if line == "" {
            lock_line = 0;
        } else {
            if lock_line == 0 {
                // determine lock type
                is_lock = if line == "#####" { true } else { false };
            } else if lock_line == 6 {
                // insert into the lock/key set
                if is_lock {
                    locks.push(current_lock_key);
                } else {
                    keys.push(current_lock_key);
                }

                // reset current_lock_key
                current_lock_key = [0; 5];
            } else {
                for (i, ch) in line.chars().enumerate() {
                    if ch == '#' {
                        current_lock_key[i] += 1;
                    }
                }
            }

            lock_line += 1;
        }
    }

    let mut answer: u64 = 0;
    for lock in locks.clone() {
        for key in keys.clone() {
            let mut no_overlaps: bool = true;
            for i in 0..5 {
                if lock[i] + key[i] > 5 {
                    no_overlaps = false;
                    break;
                }
            }

            if no_overlaps {
                answer += 1;
            }
        }
    }

    return answer;
}




#[cfg(test)] #[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test_part1a() {
        let contents: String = fs::read_to_string("src/test1a.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 3);
    }
}