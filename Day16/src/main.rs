mod position;

use std::fs;
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

    println!("part 1: {} ({:.2?})", part1, elapsed); // 111488 - too high

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

    let mut path: Vec<Position> = Vec::with_capacity(1 as usize);
    if start.x < 0 || start.y < 0 || goal.x >= mapWidth as i32 || goal.y >= mapHeight as i32 ||
       start == goal || mapWidth < 2 || mapHeight < 2 {
        return None;
    }

    /* Memory allocation */
    let mut close_set: HashSet<Position, PositionBuildHasher> = HashSet::with_capacity_and_hasher(mapHeight * mapWidth, PositionBuildHasher);
    let mut came_from: HashMap<Position, Position, PositionBuildHasher> = HashMap::with_capacity_and_hasher(mapHeight * mapWidth, PositionBuildHasher);
    let mut gscore: HashMap<Position, f32, PositionBuildHasher> = HashMap::with_capacity_and_hasher(mapHeight * mapWidth, PositionBuildHasher);
    let mut oheap: PriorityQueue<Position, OrderedFloat<f32>, PositionBuildHasher> = PriorityQueue::with_capacity_and_hasher(mapWidth + mapHeight, PositionBuildHasher);
    let mut oheap_copy: HashMap<Position, f32, PositionBuildHasher> = HashMap::with_capacity_and_hasher(mapHeight * mapWidth, PositionBuildHasher);

    let mut current: Position;
    let mut neighbors: [Position; 4];

    /* Add initial position to the search list */
    gscore.insert(start, 0.0);

    /* Note: gscore is multiplied by -1 before being entered into the oheap
     *  because of how big of a pain in the ass it is to switch it from a
     *  max heap to a min heap */
    oheap.push(start, OrderedFloat::from(-1.0*(*gscore.get(&start).unwrap_or(&0.0))));
    oheap_copy.insert(start, *gscore.get(&start).unwrap_or(&0.0));

    let mut _count: u32 = 0;
    while !oheap.is_empty() {
        _count += 1;
        (current, _) = oheap.pop().unwrap_or((Position::new(0,0), OrderedFloat::from(0.0)));
        oheap_copy.remove(&current);
        close_set.insert(current);

        neighbors = current.get_surrounding_positions();

        /* Search surrounding neighbors */
        for neighbor in neighbors {
            /* if the neighbor is a valid position */
            if neighbor.x >= 0 && neighbor.y >= 0 && 
                    neighbor.y < mapHeight as i32 && neighbor.x < mapWidth as i32 &&
                    weighted_map[neighbor.y as usize][neighbor.x as usize] < 255 {
                let neighbor_gscore: f32 = *gscore.get(&current).unwrap_or(&0.0) + 
                                            weighted_map[neighbor.y as usize][neighbor.x as usize] as f32 + 
                                            // use a default that it makes the start always point east to begin
                                            optimized_heuristic(
                                                neighbor, 
                                                current, 
                                                *came_from.get(&current)
                                                    .unwrap_or(&(start + Position::new(-1, 0)))
                                            );

                /* if the neighbor is already on the open list check to see if the neighbor is better before updating it*/
                let in_open_list: bool = oheap_copy.contains_key(&neighbor);
                if in_open_list && neighbor_gscore < *gscore.get(&neighbor).unwrap_or(&0.0){
                    /* track the node's parent */
                    came_from.insert(neighbor, current);

                    /* gscore = cost to get from the start to the current position */
                    gscore.entry(neighbor).and_modify(|val| *val = neighbor_gscore);

                    /* update the neighbors values */
                    oheap_copy.entry(neighbor).and_modify(|val| *val = neighbor_gscore);

                    /* remove the old gscore */
                    oheap.remove(&neighbor);

                    /* Add the new fscore and sort */
                    oheap.push(neighbor, OrderedFloat::from(-1.0*neighbor_gscore));
                    continue;
                }

                /* check if it is on the closed list */
                if close_set.contains(&neighbor) && neighbor_gscore < *gscore.get(&neighbor).unwrap_or(&0.0) {
                    /* remove neighbor from closed list */
                    close_set.remove(&neighbor);
                }

                /* Add to the open list */
                if !close_set.contains(&neighbor) && !in_open_list {
                    /* track the node's parent */
                    came_from.insert(neighbor, current);

                    /* gscore = cost to get rom the start to the current position */
                    gscore.insert(neighbor, neighbor_gscore);

                    /* add to the open list */
                    oheap_copy.insert(neighbor, neighbor_gscore);
                    oheap.push(neighbor, OrderedFloat::from(-1.0*neighbor_gscore));
                }
            }
        }
    }

    /* trace path back from the goal */
    current = goal;
    while current != start {
        path.push(current);
        current = *came_from.get(&current).unwrap_or(&Position::new(0,0));
    }

    path.reverse();

    // print the result
    for (row_num, row) in weighted_map.iter().enumerate() {
        for (col_num, val) in row.iter().enumerate() {
            if *val == 255 {
                print!("{}",'#');
            } else if start == Position::new(col_num as i32, row_num as i32) {
                print!("{}", 'S')
            } else if goal == Position::new(col_num as i32, row_num as i32) {
                print!("{}", 'E');
            } else if path.contains(&Position::new(col_num as i32, row_num as i32)) {
                print!("{}", 'X');
            } else {
                print!(".");
            }
        }
        println!();
    }

    return Some(gscore.get(&goal).unwrap().round() as u64);
}


#[inline]
fn optimized_heuristic(neighbor: Position, current: Position, previous: Position) -> f32 {
    let step_cost: f32 = (((neighbor.x - current.x) + (neighbor.y - current.y)) as f32).abs();
    let turn_cost: f32 = if (previous.x - neighbor.x).abs() == 1 && 
                            (previous.y - neighbor.y).abs() == 1 { 1000.0 } // turning 
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

    let mut answer: u64 = optimized_dijkstras_search(&map, start, end).unwrap_or(0);
    return answer;
}


#[allow(non_snake_case)]
fn part2(contents: String) -> u64 {
    let mut capture_map: bool = true;
    for (row_num, line) in contents.lines().enumerate() {

    }

    let mut answer: u64 = 0;
    return answer;
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
}