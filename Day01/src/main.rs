use std::fs;
use std::collections::HashMap;

#[allow(non_snake_case)]
fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    println!("part 1: {}", part1(contents.clone()));
    println!("part 2: {}", part2(contents.clone()));
}

#[allow(non_snake_case)]
fn part1(contents: String) -> u64 {
    let mut answer: u64 = 0;
    let mut left_side: Vec<u64> = vec![];
    let mut right_side: Vec<u64> = vec![];

    for line in contents.lines() {
        let both_sides: Vec<u64> = line.to_string()
            .split_ascii_whitespace()
            .map(|x: &str| x.parse::<u64>().unwrap())
            .collect();

        left_side.push(both_sides[0]);
        right_side.push(both_sides[1]);
    }

    left_side.sort_by(|a: &u64, b: &u64| b.cmp(a));
    right_side.sort_by(|a: &u64, b: &u64| b.cmp(a));

    
    for i in 0..left_side.len() {
        answer += (left_side[i] as i64 - right_side[i] as i64).abs() as u64
    }

    return answer;
}

#[allow(non_snake_case)]
fn part2(contents: String) -> u64 {
    let mut answer: u64 = 0;
    let mut left_side: Vec<u64> = vec![];
    let mut right_side: HashMap<u64, u64> = HashMap::new();

    for line in contents.lines() {
        let both_sides: Vec<u64> = line.to_string()
            .split_ascii_whitespace()
            .map(|x: &str| x.parse::<u64>().unwrap())
            .collect();

        left_side.push(both_sides[0]);

        match right_side.get(&both_sides[1]) {
            Some(x) => right_side.insert(both_sides[1], x+1),
            None => right_side.insert(both_sides[1], 1)
        };
    }

    for val in left_side.clone() {
        match right_side.get(&val) {
            Some(x) => answer += x*val,
            None => ()
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
        assert_eq!(part1(contents.clone()), 11);
    }

    #[test]
    fn test_part2() {
        let contents: String = fs::read_to_string("src/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), 31);
    }
}