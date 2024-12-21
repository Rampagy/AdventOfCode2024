mod position;

use std::fs;
use std::time::Instant;
use std::collections::{HashMap, HashSet, VecDeque};
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
    let mut path: Vec<Position> = Vec::new();

    let mut open: VecDeque<Position> = VecDeque::new();
    let mut closed: HashSet<Position, PositionBuildHasher> = HashSet::with_hasher(PositionBuildHasher);
    let mut came_from: HashMap<Position, Position, PositionBuildHasher> = HashMap::with_hasher(PositionBuildHasher);

    open.push_back(start);
    closed.insert(start);

    let mut current: Position;
    while !open.is_empty() {
        current = open.pop_front().unwrap();

        for neighbor in current.get_surrounding_positions() {
            if map[neighbor.y as usize][neighbor.x as usize] != 255 && !closed.contains(&neighbor) {
                came_from.insert(neighbor, current);
                open.push_back(neighbor);
                closed.insert(neighbor);

                if neighbor == end {
                    // trace back and return the path

                    current = end;
                    while current != start {
                        path.push(current);
                        current = *came_from.get(&current).unwrap();
                    }
                    path.push(current);
                    path.reverse();
                    return path;
                }
            }
        }
    }

    return path;
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

    // get a path with no cheats
    let path: Vec<Position> = bfs(&map, start, end);

    let mut valid_skips:u64 = 0;
    // go through each position on the path and check if it's a valid skip
    for (pindex, p) in path.iter().enumerate() {
        for offset in p.get_directions() {
            let neighbor: Position = *p + offset;
            if map[neighbor.y as usize][neighbor.x as usize] == 255 {
                let neighbors_neighbor: Position = neighbor + offset;
                if neighbors_neighbor.x >= 0 && neighbors_neighbor.y >= 0 && 
                    neighbors_neighbor.x < map[0].len() as i32 && neighbors_neighbor.y < map.len() as i32 && 
                    map[neighbors_neighbor.y as usize][neighbors_neighbor.x as usize] != 255 {
                        // jump found (could be backwards)
                        let nn_index: usize = path.iter().position(|&x| x == neighbors_neighbor).unwrap();
                        if nn_index > pindex {
                            // valid forward jump
                            let delta: usize = nn_index - pindex - 2;
                            if delta >= time_saved as usize {
                                valid_skips += 1;
                            }
                        }
                }
            }
        }
    }

    return 0;
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