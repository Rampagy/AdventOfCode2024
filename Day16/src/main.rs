mod position;

use std::{fs, u64};
use std::time::Instant;
use position::{Position, PositionBuildHasher};
use std::collections::{HashSet, HashMap};
use priority_queue::PriorityQueue;
use ordered_float::OrderedFloat;


#[allow(non_snake_case)]
fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");
    let mut now: Instant;
    let mut elapsed: std::time::Duration;

    now = Instant::now();
    let part1: u64 = part1(contents.clone());
    elapsed = now.elapsed();

    println!("part 1: {} ({:.2?})", part1, elapsed);

    now = Instant::now();
    let part2: u64 = part2(contents.clone());
    elapsed = now.elapsed();

    println!("part 2: {} ({:.2?})", part2, elapsed);
}


#[allow(non_snake_case)]
pub fn optimized_dijkstras_search(  weighted_map: &Vec<Vec<u8>>, start: Position, 
                                    goal: Position ) -> Option<u64> {
    let mapWidth: usize = weighted_map[0].len();
    let mapHeight: usize = weighted_map.len();

    if start.x < 0 || start.y < 0 || goal.x >= mapWidth as i32 || goal.y >= mapHeight as i32 ||
       start == goal || mapWidth < 2 || mapHeight < 2 {
        return None;
    }

    /* Memory allocation */
    let mut close_set: HashSet<(Position, Position)> = HashSet::with_capacity(mapHeight * mapWidth);
    let mut came_from: HashMap<(Position, Position), (Position, Position)> = HashMap::with_capacity(mapHeight * mapWidth);
    let mut gscore: HashMap<(Position, Position), f32> = HashMap::with_capacity(mapHeight * mapWidth);
    let mut oheap: PriorityQueue<(Position, Position), OrderedFloat<f32>> = PriorityQueue::with_capacity(mapWidth + mapHeight);
    let mut oheap_copy: HashMap<(Position, Position), f32> = HashMap::with_capacity(mapHeight * mapWidth);

    let mut current: Position = start;
    let mut neighbors: [Position; 4];
    let mut current_direction: Position = Position::new(1, 0);

    /* Add initial position to the search list */
    gscore.insert((current, current_direction), 0.0);

    /* Note: gscore is multiplied by -1 before being entered into the oheap
     *  because of how big of a pain in the ass it is to switch it from a
     *  max heap to a min heap */
    oheap.push((start, Position::new(1, 0)), 
                OrderedFloat::from(-1.0*(*gscore.get(&(current, current_direction)).unwrap_or(&0.0))));
    oheap_copy.insert((start, Position::new(1, 0)), 
                *gscore.get(&(current, current_direction)).unwrap_or(&0.0));

    let mut _count: u32 = 0;
    while !oheap.is_empty() {
        _count += 1;
        ((current, current_direction), _) = oheap.pop().unwrap();
        oheap_copy.remove(&(current, current_direction));
        close_set.insert((current, current_direction));

        if current == goal {
            // exit
            break;
        }

        /* Search surrounding neighbors */
        neighbors = current.get_surrounding_positions();
        for neighbor in neighbors {

            /* if the neighbor is a valid position */
            if neighbor.x >= 0 && neighbor.y >= 0 && 
                    neighbor.y < mapHeight as i32 && neighbor.x < mapWidth as i32 &&
                    weighted_map[neighbor.y as usize][neighbor.x as usize] < 255 {

                // track the neighbors current direction
                let neighbor_direction: Position = neighbor - current;
                let neighbor_gscore: f32 = *gscore.get(&(current, current_direction)).unwrap_or(&0.0) + 
                                            weighted_map[neighbor.y as usize][neighbor.x as usize] as f32 + 
                                            // use a default that it makes the start always point east to begin
                                            optimized_heuristic(
                                                neighbor, 
                                                current, 
                                                came_from.get(&(current, current_direction))
                                                    .unwrap_or(&(start + Position::new(-1, 0), Position::new(-1, 0))).0
                                            );

                /* if the neighbor is already on the open list check to see if the neighbor is better before updating it */
                let in_open_list: bool = oheap_copy.contains_key(&(neighbor, neighbor_direction));
                if in_open_list && neighbor_gscore < *gscore.get(&(neighbor, neighbor_direction)).unwrap_or(&0.0){
                    /* track the node's parent */
                    came_from.insert((neighbor, neighbor_direction), (current, current_direction));

                    /* gscore = cost to get from the start to the current position */
                    gscore.entry((neighbor, neighbor_direction)).and_modify(|val| *val = neighbor_gscore);

                    /* update the neighbors values */
                    oheap_copy.entry((neighbor, neighbor_direction)).and_modify(|val| *val = neighbor_gscore);

                    /* remove the old gscore */
                    oheap.remove(&(neighbor, neighbor_direction));

                    /* Add the new fscore and sort */
                    oheap.push((neighbor, neighbor_direction), OrderedFloat::from(-1.0*neighbor_gscore));
                    continue;
                }

                /* check if it is on the closed list */
                if close_set.contains(&(neighbor, neighbor_direction)) && neighbor_gscore < *gscore.get(&(neighbor, neighbor_direction)).unwrap_or(&0.0) {
                    /* remove neighbor from closed list */
                    close_set.remove(&(neighbor, neighbor_direction));
                }

                /* Add to the open list */
                if !close_set.contains(&(neighbor, neighbor_direction)) && !in_open_list {
                    /* track the node's parent */
                    came_from.insert((neighbor, neighbor_direction), (current, current_direction));

                    /* gscore = cost to get rom the start to the current position */
                    gscore.insert((neighbor, neighbor_direction), neighbor_gscore);

                    /* add to the open list */
                    oheap_copy.insert((neighbor, neighbor_direction), neighbor_gscore);
                    oheap.push((neighbor, neighbor_direction), OrderedFloat::from(-1.0*neighbor_gscore));
                }
            }
        }
    }

    return Some(gscore.get(&(current, current_direction)).unwrap().round() as u64);
}


#[inline]
fn optimized_heuristic(neighbor: Position, current: Position, previous: Position) -> f32 {
    let step_cost: f32 = (((neighbor.x - current.x) + (neighbor.y - current.y)) as f32).abs();
    let turn_cost: f32 = if (previous.x - neighbor.x).abs() == 1 && 
                            (previous.y - neighbor.y).abs() == 1 { 1000.0 } // turning 
                        else if previous == neighbor { 2000.0 } // did a 180
                        else { 0.0 }; // straight

    return step_cost + turn_cost;
}


#[allow(non_snake_case)]
fn part1(contents: String) -> u64 {
    let mut map: Vec<Vec<u8>> = Vec::new();
    let mut start: Position = Position::new(0,0);
    let mut end: Position = Position::new(0,0);

    for (row_num, line) in contents.lines().enumerate() {
        let mut row: Vec<u8> = Vec::new();
        for (col_num, c) in line.chars().enumerate() {
            if c == '#' {
                row.push(255);
            } else {
                row.push(0);
            }

            if c == 'S' {
                start = Position::new(col_num as i32, row_num as i32);
            } else if c == 'E' {
                end = Position::new(col_num as i32, row_num as i32);
            }
        }
        map.push(row);
    }

    return optimized_dijkstras_search(&map, start, end).unwrap_or(0);
}


#[allow(non_snake_case)]
pub fn optimized_dijkstras_search_p2(weighted_map: &Vec<Vec<u8>>, start: Position, goal: Position) -> Option<Vec<Vec<Position>>> {
    let mapWidth: usize = weighted_map[0].len();
    let mapHeight: usize = weighted_map.len();

    if start.x < 0 || start.y < 0 || goal.x >= mapWidth as i32 || goal.y >= mapHeight as i32 ||
       start == goal || mapWidth < 2 || mapHeight < 2 {
        return None;
    }

    /* Memory allocation */
    let mut close_set: HashSet<(Position, Position)> = HashSet::with_capacity(mapHeight * mapWidth);
    let mut came_from: HashMap<(Position, Position), Vec<(Position, Position)>> = HashMap::with_capacity(mapHeight * mapWidth);
    let mut gscore: HashMap<(Position, Position), f32> = HashMap::with_capacity(mapHeight * mapWidth);
    let mut oheap: PriorityQueue<(Position, Position), OrderedFloat<f32>> = PriorityQueue::with_capacity(mapWidth + mapHeight);
    let mut oheap_copy: HashMap<(Position, Position), f32> = HashMap::with_capacity(mapHeight * mapWidth);

    let mut current: Position = start;
    let mut neighbors: [Position; 4];
    let mut current_direction: Position = Position::new(1, 0);

    /* Add initial position to the search list */
    gscore.insert((current, current_direction), 0.0);

    /* Note: gscore is multiplied by -1 before being entered into the oheap
     *  because of how big of a pain in the ass it is to switch it from a
     *  max heap to a min heap */
    oheap.push((start, Position::new(1, 0)), 
                OrderedFloat::from(-1.0*(*gscore.get(&(current, current_direction)).unwrap_or(&0.0))));
    oheap_copy.insert((start, Position::new(1, 0)), 
                *gscore.get(&(current, current_direction)).unwrap_or(&0.0));

    let mut _count: u32 = 0;
    while !oheap.is_empty() {
        _count += 1;
        ((current, current_direction), _) = oheap.pop().unwrap();
        oheap_copy.remove(&(current, current_direction));
        close_set.insert((current, current_direction));

        if current == goal {
            // exit
            break;
        }

        /* Search surrounding neighbors */
        neighbors = current.get_surrounding_positions();
        for neighbor in neighbors {

            /* if the neighbor is a valid position */
            if neighbor.x >= 0 && neighbor.y >= 0 && 
                    neighbor.y < mapHeight as i32 && neighbor.x < mapWidth as i32 &&
                    weighted_map[neighbor.y as usize][neighbor.x as usize] < 255 {

                // track the neighbors current direction
                let neighbor_direction: Position = neighbor - current;
                let neighbor_gscore: f32 = *gscore.get(&(current, current_direction)).unwrap_or(&0.0) + 
                                            weighted_map[neighbor.y as usize][neighbor.x as usize] as f32 + 
                                            // use a default that it makes the start always point east to begin
                                            optimized_heuristic(
                                                neighbor, 
                                                current, 
                                                came_from.get(&(current, current_direction))
                                                    .unwrap_or(&vec![(start + Position::new(-1, 0), Position::new(-1, 0))]).first().unwrap().0
                                            );

                /* if the neighbor is already on the open list check to see if the neighbor is better before updating it */
                let in_open_list: bool = oheap_copy.contains_key(&(neighbor, neighbor_direction));
                if in_open_list && neighbor_gscore <= *gscore.get(&(neighbor, neighbor_direction)).unwrap_or(&0.0){
                    /* track the node's parent */
                    came_from.entry((neighbor, neighbor_direction)).or_insert_with(Vec::new).push((current, current_direction));

                    /* gscore = cost to get from the start to the current position */
                    gscore.entry((neighbor, neighbor_direction)).and_modify(|val| *val = neighbor_gscore);

                    /* update the neighbors values */
                    oheap_copy.entry((neighbor, neighbor_direction)).and_modify(|val| *val = neighbor_gscore);

                    /* remove the old gscore */
                    oheap.remove(&(neighbor, neighbor_direction));

                    /* Add the new fscore and sort */
                    oheap.push((neighbor, neighbor_direction), OrderedFloat::from(-1.0*neighbor_gscore));
                    continue;
                }

                /* check if it is on the closed list */
                if close_set.contains(&(neighbor, neighbor_direction)) && neighbor_gscore <= *gscore.get(&(neighbor, neighbor_direction)).unwrap_or(&0.0) {
                    /* remove neighbor from closed list */
                    close_set.remove(&(neighbor, neighbor_direction));
                }

                /* Add to the open list */
                if !close_set.contains(&(neighbor, neighbor_direction)) && !in_open_list {
                    /* track the node's parent */
                    came_from.entry((neighbor, neighbor_direction)).or_insert_with(Vec::new).push((current, current_direction));

                    /* gscore = cost to get rom the start to the current position */
                    gscore.insert((neighbor, neighbor_direction), neighbor_gscore);

                    /* add to the open list */
                    oheap_copy.insert((neighbor, neighbor_direction), neighbor_gscore);
                    oheap.push((neighbor, neighbor_direction), OrderedFloat::from(-1.0*neighbor_gscore));
                }
            }
        }
    }

    // Reconstruct all paths
    let mut paths: Vec<Vec<Position>> = Vec::new();
    let mut stack: Vec<(Position, Position, Vec<Position>)> = Vec::new();
    stack.push((goal, Position::new(0, -1), vec![goal])); // north
    stack.push((goal, Position::new(1, 0), vec![goal])); // east
    stack.push((goal, Position::new(0, 1), vec![goal])); // south
    stack.push((goal, Position::new(-1, 0), vec![goal])); // west

    while !stack.is_empty() {
        let stack_option: Option<(Position, Position, Vec<Position>)> = stack.pop();
        if stack_option.is_some() {
            let (current_position, direction, path) = stack_option.unwrap();
            let parent_option: Option<&Vec<(Position, Position)>> = came_from.get(&(current_position, direction));
            if current_position == start {
                // position == start
                paths.push(path.clone());
            } else if parent_option.is_some() {
                for &(parent, parent_direction) in parent_option.unwrap() {
                    let mut new_path: Vec<Position> = path.clone();
                    new_path.push(parent);
                    stack.push((parent, parent_direction, new_path));
                }
            }
        }
    }

    return Some(paths);
}


#[allow(non_snake_case)]
fn part2(contents: String) -> u64 {
    let mut map: Vec<Vec<u8>> = Vec::new();
    let mut start: Position = Position::new(0,0);
    let mut end: Position = Position::new(0,0);

    for (row_num, line) in contents.lines().enumerate() {
        let mut row: Vec<u8> = Vec::new();
        for (col_num, c) in line.chars().enumerate() {
            if c == '#' {
                row.push(255);
            } else {
                row.push(0);
            }

            if c == 'S' {
                start = Position::new(col_num as i32, row_num as i32);
            } else if c == 'E' {
                end = Position::new(col_num as i32, row_num as i32);
            }
        }
        map.push(row);
    }

    let paths: Option<Vec<Vec<Position>>> = optimized_dijkstras_search_p2(&map, start, end);
    let mut shortest_path: u64 = u64::MAX;
    let mut path_positions: HashSet<Position, PositionBuildHasher> = HashSet::with_hasher(PositionBuildHasher);
    if paths.is_some() {
        for path in paths.unwrap() {
            if (path.len() as u64) < shortest_path {
                shortest_path = path.len() as u64;
                path_positions.clear();
            }
            
            if (path.len() as u64) == shortest_path {
                // add all unique path positions
                path_positions.extend(path);
            }
        }
    }

    return path_positions.len() as u64;
}


#[cfg(test)] #[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test_part1a() {
        let contents: String = fs::read_to_string("src/test1a.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 7036);
    }

    #[test]
    fn test_part1b() {
        let contents: String = fs::read_to_string("src/test1b.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 11048);
    }

    #[test]
    fn test_part1c() {
        let contents: String = fs::read_to_string("src/test1c.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 3009);
    }

    #[test]
    fn test_part1d() {
        let contents: String = fs::read_to_string("src/test1d.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 2009);
    }

    #[test]
    fn test_part1e() {
        let contents: String = fs::read_to_string("src/test1e.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 6022);
    }

    #[test]
    fn test_part1f() {
        let contents: String = fs::read_to_string("src/test1f.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 10040);
    }

    #[test]
    fn test_part2a() {
        let contents: String = fs::read_to_string("src/test2a.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), 45);
    }

    #[test]
    fn test_part2b() {
        let contents: String = fs::read_to_string("src/test2b.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), 64);
    }
}