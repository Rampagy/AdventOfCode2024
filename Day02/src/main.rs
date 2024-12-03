use std::fs;

#[allow(non_snake_case)]
fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    println!("part 1: {}", part1(contents.clone()));
    println!("part 2: {}", part2(contents.clone()));
}

#[allow(non_snake_case)]
fn part1(contents: String) -> u64 {
    let mut answer: u64 = 0;

    for line in contents.lines() {
        let report: Vec<u64> = line.to_string()
            .split_ascii_whitespace()
            .map(|x: &str| x.parse::<u64>().unwrap())
            .collect();

        let mut level_delta: i64 = 0;
        for i in 1..report.len() {
            let delta: i64 = report[i] as i64 - report[i-1] as i64;
            if delta > 0 && delta < 4 {
                level_delta += 1;
            } else if delta < 0 && delta > -4 {
                level_delta -= 1;
            }
        }

        if level_delta.abs() == (report.len()-1) as i64 {
            answer += 1;
        }
    }

    return answer;
}

#[allow(non_snake_case)]
fn check_good(report: Vec<u64>) -> bool {
    let mut level_delta: i64 = 0;
        for i in 1..report.len() {
            let delta: i64 = report[i] as i64 - report[i-1] as i64;
            if delta > 0 && delta < 4 {
                level_delta += 1;
            } else if delta < 0 && delta > -4 {
                level_delta -= 1;
            }
        }

        if level_delta.abs() as usize == report.len()-1 {
            return true;
        } else {
            return false;
        }
}

#[allow(non_snake_case)]
fn part2(contents: String) -> u64 {
    let mut answer: u64 = 0;

    for line in contents.lines() {
        let report: Vec<u64> = line.to_string()
            .split_ascii_whitespace()
            .map(|x: &str| x.parse::<u64>().unwrap())
            .collect();

        if check_good(report.clone()) {
            answer += 1;
        } else {
            for i in 0..report.len() {
                let mut new_report: Vec<u64> = report.clone();
                new_report.remove(i);
                if check_good(new_report) {
                    answer += 1;
                    break;
                }
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
        assert_eq!(part1(contents.clone()), 2);
    }

    #[test]
    fn test_part2() {
        let contents: String = fs::read_to_string("src/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), 4);
    }
}