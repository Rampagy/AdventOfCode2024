use std::fs;
use std::time::Instant;
use itertools::Itertools;


#[allow(non_snake_case)]
fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    let mut now: Instant = Instant::now();
    let part1: u64 = part1(contents.clone());
    let mut elapsed: std::time::Duration = now.elapsed();

    println!("part 1: {} ({:.2?})", part1, elapsed);

    now = Instant::now();
    let part2: u64 = part2(contents.clone());
    elapsed = now.elapsed();

    println!("part 2: {} ({:.2?})", part2, elapsed);
}


#[allow(non_snake_case)]
fn part1(contents: String) -> u64 {
    let mut answer: u64 = 0;

    for (_line_num, line) in contents.lines().enumerate() {
        let total: u64 = line.split(':')
            .nth(0).unwrap()
            .parse::<u64>().unwrap();
        let coefficients: Vec<u64> = line.to_string()
            .split(':')
            .nth(1).unwrap()
            .split_ascii_whitespace()
            .map(|x: &str| x.parse::<u64>().unwrap())
            .collect();


        let mut loop_count: u64 = 0;
        while loop_count < (1 << coefficients.len()) {
            let mut current_evaluation: u64 = coefficients[0];
            for i in 1..coefficients.len() {
                // check each bit in loop count to see if we should add or multiply
                if ((1 << i) & loop_count) == 0 {
                    // 0b0 is an add
                    current_evaluation += coefficients[i];
                } else {
                    // 0b1 is a multiply
                    current_evaluation *= coefficients[i];
                }
            }

            if current_evaluation == total {
                answer += total;
                break;
            }

            loop_count = loop_count.saturating_add(1);
        }
    }

    return answer;
}

fn concat(a: u64, b: u64) -> u64 { 
    return a as u64 * 10u64.pow(b.ilog10() + 1) + b as u64;
}


#[allow(non_snake_case)]
fn part2(contents: String) -> u64 {
    let mut answer: u64 = 0;

    for (_line_num, line) in contents.lines().enumerate() {
        let total: u64 = line.split(':')
            .nth(0).unwrap()
            .parse::<u64>().unwrap();
        let coefficients: Vec<u64> = line.to_string()
            .split(':')
            .nth(1).unwrap()
            .split_ascii_whitespace()
            .map(|x: &str| x.parse::<u64>().unwrap())
            .collect();

        // (number of values to select).map(|| range of values to select from).generate permutations with repition
        for perm in (0..coefficients.len()-1).map(|_| 0..3).multi_cartesian_product() {
            let mut current_evaluation: u64 = coefficients[0];
            for (n, p) in perm.iter().enumerate() {
                // check each bit in loop count to see if we should add or multiply
                if *p == 0 {
                    // remainder of 0 is add
                    current_evaluation += coefficients[n+1];
                } else if *p == 1 {
                    // remainder of 1 is multiply
                    current_evaluation *= coefficients[n+1];
                } else {
                    // remainder of 2 is concatenate
                    current_evaluation = concat(current_evaluation, coefficients[n+1]);
                }

                if current_evaluation > total {
                    break;
                }
            }

            if current_evaluation == total {
                answer += total;
                break;
            }
        }
    }

    return answer;
}


#[cfg(test)] #[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 3749);
    }

    #[test]
    fn test_part2() {
        let contents: String = fs::read_to_string("src/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), 11387);
    }
}