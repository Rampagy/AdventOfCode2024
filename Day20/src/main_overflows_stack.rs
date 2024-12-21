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



fn depth_first_search(map: &Vec<Vec<u8>>, start: Position, end: Position, current: Position, 
                    allow_jumps: bool, came_from: &mut HashMap<Position, Position, PositionBuildHasher>, 
                    cache: &mut HashMap<Position, u64, PositionBuildHasher>, path_length: u64, cheat_paths: &mut u64) -> u64 {
    for direction in [Position::new(0, -1),  // north
                                Position::new( 1, 0),  // east
                                Position::new( 0, 1),  // south
                                Position::new(-1, 0)] { // west
        let neighbor: Position = current + direction;
        if neighbor != end {
            if map[neighbor.y as usize][neighbor.x as usize] != 255 && Some(&neighbor) != came_from.get(&current) {
                // valid direction
                came_from.insert(neighbor, current);
                let answer: u64 = depth_first_search(map, start, end, neighbor, allow_jumps, came_from, cache, path_length, cheat_paths);
                if !allow_jumps {
                    return answer;
                } else {
                    // continue the search
                }
            } else if allow_jumps && Some(&neighbor) != came_from.get(&current) && map[neighbor.y as usize][neighbor.x as usize] == 255 {
                // potential jump
                let neighbors_neighbor: Position = neighbor + direction;
                if neighbors_neighbor.x >= 0 && neighbors_neighbor.y >= 0 && 
                        neighbors_neighbor.x < map[0].len() as i32 && neighbors_neighbor.y < map.len() as i32 && 
                        map[neighbors_neighbor.y as usize][neighbors_neighbor.x as usize] != 255 {
                    let current_steps: u64 = *cache.get(&current).unwrap();
                    let neighbors_neighbor_steps: u64 = *cache.get(&neighbors_neighbor).unwrap();

                    // check for backwards jumps
                    if neighbors_neighbor_steps < current_steps {
                        // forward jump

                        // get the stesps from neighbors_neighbor to end
                        let mut steps: u64 = neighbors_neighbor_steps;

                        // get the steps from the start to the current
                        let mut trace_back: Position = current;
                        let mut path: Vec<Position> = Vec::new();
                        while trace_back != start {
                            // add to path
                            path.push(trace_back);
            
                            // get parent
                            trace_back = *came_from.get(&trace_back).unwrap_or(&Position::new(0, 0));
            
                            // increment locations from the end
                            steps += 1;
                        }

                        // add 2 more steps for the jump
                        steps += 2;

                        if steps <= path_length {
                            // a valid path that has removed the specified amount of steps
                            *cheat_paths += 1;
                        }
                    }
                }
            }
        } else {
            came_from.insert(neighbor, current);

            // at the end - trace back the path, update cache, and then return the length of the path
            let mut trace_back: Position = end;
            let mut path: Vec<Position> = Vec::new();
            let mut count: u64 = 0;
            while trace_back != start {
                // add to path
                path.push(trace_back);

                if !allow_jumps {
                    // only update cache if jumps are not allowed
                    cache.insert(trace_back, count);
                }

                // get parent
                trace_back = *came_from.get(&trace_back).unwrap();

                // increment locations from the end
                count += 1;
            }

            // add start to the cache
            if !allow_jumps {
                // only update cache if jumps are not allowed
                cache.insert(trace_back, count);
            }

            if count <= path_length {
                // a valid path that has removed the specified amount of steps
                *cheat_paths += 1;
            }

            return count;
        }
    }

    return 0;
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

    let longest_path: u64 = depth_first_search(&map, start, end, start, false, 
                                        &mut HashMap::with_hasher(PositionBuildHasher), 
                                        &mut cache, 0, &mut cheat_paths) - 1;

    // now we know the longest path - search for the rest of the paths
    cheat_paths = 0;
    depth_first_search(&map, start, end, start, true, 
                    &mut HashMap::with_hasher(PositionBuildHasher), 
                    &mut cache, longest_path-time_saved, &mut cheat_paths);

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