mod position;

use std::fs;
use std::time::Instant;
use std::collections::{HashMap, HashSet};
use position::Position;
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
    let mut antennas: HashMap<char, Vec<Position>> = HashMap::new();
    let mut raw_antennas: HashSet<Position> = HashSet::new();
    let mut map_width: u64 = 0;
    let mut map_height: u64 = 0;
    let mut antinodes: HashMap<char, Vec<Position>> = HashMap::new();

    for (row_num, line) in contents.lines().enumerate() {
        for (col_num, c) in line.chars().enumerate() {
            if c != '.' {
                // found antenna, add to the map
                let new_antenna: Position = Position::new( col_num as i32, row_num as i32);
                match antennas.get_mut(&c) {
                    Some(v) => {
                        v.push(new_antenna);
                    }
                    None => {
                        antennas.insert(c, vec![new_antenna]);
                    }
                }

                if !raw_antennas.contains(&new_antenna) {
                    raw_antennas.insert(new_antenna);
                }
            }

            if col_num as u64 > map_width {
                map_width = col_num as u64;
            }
        }

        if row_num as u64 > map_height {
            map_height = row_num as u64;
        }
    }

    // loop through each antenna type
    for (frequency, antenna_locs) in antennas.iter() {
        // (number of values to select).map(|| range of values to select from).generate permutations with repition
        for perm in (0..antenna_locs.len()).permutations(2) {
            let dx = antenna_locs[perm[0]].x - antenna_locs[perm[1]].x;
            let dy = antenna_locs[perm[0]].y - antenna_locs[perm[1]].y;

            let new_antinodes: [Position; 2] = [
                Position::new(antenna_locs[perm[0]].x + dx, antenna_locs[perm[0]].y + dy),
                Position::new(antenna_locs[perm[1]].x - dx, antenna_locs[perm[1]].y - dy)
            ];

            for antinode in new_antinodes {
                if ((antinodes.contains_key(&frequency) && !antinodes.get(frequency).unwrap().contains(&antinode)) || !antinodes.contains_key(&frequency)) && 
                        antinode.x <= map_width as i32 && antinode.y <= map_height as i32 && 
                        antinode.x >= 0 && antinode.y >= 0 {
                    // not already an antinode fo rthat frequency, not an antenna, and inside the map
                    // add to antinodes
                    match antinodes.get_mut(&frequency) {
                        Some(v) => {
                            v.push(antinode);
                        }
                        None => {
                            antinodes.insert(*frequency, vec![antinode]);
                        }
                    }
                }
            }
        }
    }

    // draw antinodes
    for i in 0..=map_height {
        let mut out_line: String = "".to_string();
        for j in 0..=map_width {
            let mut added: bool = false;
            for k in antinodes.keys() {
                if antinodes[k].contains(&Position::new(j as i32, i as i32)) {
                    out_line += "#";
                    added = true;
                    answer += 1;
                    break;
                }
            }
            if !added {
                out_line += ".";
            }
        }
        println!("{}", out_line);
    }

    return answer;
}


#[allow(non_snake_case)]
fn part2(contents: String) -> u64 {
    let mut answer: u64 = 0;
    let mut antennas: HashMap<char, Vec<Position>> = HashMap::new();
    let mut raw_antennas: HashSet<Position> = HashSet::new();
    let mut map_width: u64 = 0;
    let mut map_height: u64 = 0;
    let mut antinodes: HashMap<char, Vec<Position>> = HashMap::new();

    for (row_num, line) in contents.lines().enumerate() {
        for (col_num, c) in line.chars().enumerate() {
            if c != '.' {
                // found antenna, add to the map
                let new_antenna: Position = Position::new( col_num as i32, row_num as i32);
                match antennas.get_mut(&c) {
                    Some(v) => {
                        v.push(new_antenna);
                    }
                    None => {
                        antennas.insert(c, vec![new_antenna]);
                    }
                }

                if !raw_antennas.contains(&new_antenna) {
                    raw_antennas.insert(new_antenna);
                }
            }

            if col_num as u64 > map_width {
                map_width = col_num as u64;
            }
        }

        if row_num as u64 > map_height {
            map_height = row_num as u64;
        }
    }

    // loop through each antenna type
    for (frequency, antenna_locs) in antennas.iter() {
        // (number of values to select).map(|| range of values to select from).generate permutations with repition
        for perm in (0..antenna_locs.len()).permutations(2) {
            let dx = antenna_locs[perm[0]].x - antenna_locs[perm[1]].x;
            let dy = antenna_locs[perm[0]].y - antenna_locs[perm[1]].y;

            for direction in [-1, 1] {
                let mut antinode: Position = Position::new(antenna_locs[perm[0]].x + (direction*dx), antenna_locs[perm[0]].y + (direction*dy));

                while antinode.x >= 0 && antinode.y >= 0 && antinode.x <= map_width as i32 && antinode.y <= map_height as i32 {
                    if ((antinodes.contains_key(&frequency) && !antinodes.get(frequency).unwrap().contains(&antinode)) || !antinodes.contains_key(&frequency)) && 
                            antinode.x <= map_width as i32 && antinode.y <= map_height as i32 && 
                            antinode.x >= 0 && antinode.y >= 0 {
                        // not already an antinode fo rthat frequency, not an antenna, and inside the map
                        // add to antinodes
                        match antinodes.get_mut(&frequency) {
                            Some(v) => {
                                v.push(antinode);
                            }
                            None => {
                                antinodes.insert(*frequency, vec![antinode]);
                            }
                        }
                    }

                    // update antinode
                    antinode.x += direction*dx;
                    antinode.y += direction*dy;
                }
            }
        }
    }

    // draw antinodes
    for i in 0..=map_height {
        let mut out_line: String = "".to_string();
        for j in 0..=map_width {
            let mut added: bool = false;
            for k in antinodes.keys() {
                if antinodes[k].contains(&Position::new(j as i32, i as i32)) {
                    out_line += "#";
                    added = true;
                    answer += 1;
                    break;
                }
            }
            if !added {
                out_line += ".";
            }
        }
        println!("{}", out_line);
    }

    return answer;
}


#[cfg(test)] #[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 14);
    }

    #[test]
    fn test_part2() {
        let contents: String = fs::read_to_string("src/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), 34);
    }
}