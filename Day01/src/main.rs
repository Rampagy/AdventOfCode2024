use std::fs;

#[allow(non_snake_case)]
fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    println!("part 1: {}", part1(contents.clone()));
    println!("part 2: {}", part2(contents.clone()));
}


fn part1(contents: String) -> u32 {
    let mut somearray: Vec<char> = Vec::new();

    for (row, line) in contents.lines().enumerate() {

    }

    return 0;
}

fn part2(contents: String) -> u32 {
    let mut somearray: Vec<char> = Vec::new();

    for (row, line) in contents.lines().enumerate() {

    }

    return 0;
}