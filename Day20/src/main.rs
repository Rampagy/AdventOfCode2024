mod position;

use std::fs;
use std::time::Instant;
use std::collections::HashMap;
use position::{Position, PositionBuildHasher};

#[allow(non_snake_case)]
fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");
    let mut now: Instant;
    let mut elapsed: std::time::Duration;

    now = Instant::now();
    let part1: u64 = part1(contents.clone(), 100);
    elapsed = now.elapsed();

    println!("part 1: {} ({:.2?})", part1, elapsed);

    now = Instant::now();
    let part2: u64 = part2(contents.clone());
    elapsed = now.elapsed();

    println!("part 2: {} ({:.2?})", part2, elapsed);
}

fn bfs (map: &Vec<Vec<u8>>, start: Position, end: Position) -> Vec<Position> {
    // TODO
    return Vec::new();
}

#[allow(non_snake_case)]
fn part1(contents: String, time_saved: u64) -> u64 {
    let mut map: Vec<Vec<u8>> = Vec::new();
    let mut start: Position = Position::new(-1, -1);
    let mut end: Position = Position::new(-1, -1);

    for (row_num, line) in contents.lines().enumerate() {
        let mut row: Vec<u8> = Vec::new();
        for (col_num, c) in line.chars().enumerate() {
            if c == '#' {
                // unwalkable
                row.push(255);
            } else if c == 'S' {
                // start
                row.push(0);
                start.x = col_num as i32;
                start.y = row_num as i32;
            } else if c == 'E' {
                row.push(0);
                end.x = col_num as i32;
                end.y = row_num as i32;
            } else {
                row.push(0);
            }
        }
        map.push(row);
    }

    let mut cache: HashMap<Position, u64, PositionBuildHasher> = HashMap::with_hasher(PositionBuildHasher);
    let mut cheat_paths: u64 = 0;

    // get a path with no cheats
    let path: Vec<Position> = bfs(&map, start, end);


    return cheat_paths;
}



#[allow(non_snake_case)]
fn part2(contents: String) -> u64 {


    for (_line_num, line) in contents.lines().enumerate() {
        
    }

    return 0;
}



#[cfg(test)] #[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test_part1a() {
        let contents: String = fs::read_to_string("src/test1a.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone(), 5), 16);
    }

    #[test]
    fn test_part1b() {
        let contents: String = fs::read_to_string("src/test1a.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone(), 1), 44);
    }

    #[test]
    fn test_part2a() {
        let contents: String = fs::read_to_string("src/test2a.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), 16);
    }
}