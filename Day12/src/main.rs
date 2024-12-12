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
    let part1: u64 = part1(contents.clone());
    elapsed = now.elapsed();

    println!("part 1: {} ({:.2?})", part1, elapsed);

    now = Instant::now();
    let part2: u64 = part2(contents.clone());
    elapsed = now.elapsed();

    println!("part 2: {} ({:.2?})", part2, elapsed);
}

struct GardenPlot {
    perimeter: u64,
    area: u64,
    locations: HashSet<Position, PositionBuildHasher>,
}

impl GardenPlot {
    fn new(p: u64, a: u64, l: HashSet<Position, PositionBuildHasher>) -> Self {
        Self {perimeter: p, area: a, locations: l}
    }
}

#[allow(non_snake_case)]
fn BreadthFirstSearch(map: &Vec<Vec<char>>, start: Position, garden: &mut HashMap<char, Vec<GardenPlot>>, plant: char) -> () {
    let map_height: u64 = map.len() as u64;
    let map_width: u64 = map.first().unwrap().len() as u64;

    let mut open: VecDeque<Position> = VecDeque::new();
    let mut closed: HashSet<Position, PositionBuildHasher> = HashSet::with_hasher(PositionBuildHasher);

    // add a new plot
    match garden.get_mut(&plant) {
        Some(x) => {
            // push a new garden plot
            let new_locations: HashSet<Position, PositionBuildHasher> = HashSet::with_hasher(PositionBuildHasher);
            let new_plot: GardenPlot = GardenPlot::new(0, 0, new_locations);
            x.push(new_plot);
        } None => {
            // insert the first garden plot
            let new_locations: HashSet<Position, PositionBuildHasher> = HashSet::with_hasher(PositionBuildHasher);
            let new_plot: GardenPlot = GardenPlot::new(0, 0, new_locations);
            garden.insert(plant, vec![new_plot]);
        }
    }

    open.push_back(start);
    while !open.is_empty() {
        let current: Position = open.pop_front().unwrap();
        closed.insert(current);

        match garden.get_mut(&plant) {
            Some(x) => {
                // increment area and add to the locations
                x.last_mut().unwrap().area += 1;
                x.last_mut().unwrap().locations.insert(current);
            } None => {
                // should never be reached, but insert just in case
                let mut locations: HashSet<Position, PositionBuildHasher> = HashSet::with_hasher(PositionBuildHasher);
                locations.insert(current);
                garden.insert(plant, vec![GardenPlot::new(0, 1, locations)]);
            }
        }

        for neighbor in current.get_surrounding_positions() {
            if !closed.contains(&neighbor) {
                if neighbor.x >= 0 && neighbor.y >= 0 && neighbor.x < map_width as i32 && neighbor.y < map_height as i32 && 
                        map[neighbor.y as usize][neighbor.x as usize] == plant {

                    if !open.contains(&neighbor) {
                        open.push_back(neighbor);
                    }

                } else {
                    // perimeter reached
                    match garden.get_mut(&plant) {
                        Some(x) => {
                            // increment perimeter
                            x.last_mut().unwrap().perimeter += 1;
                        } None => {
                            // should never be reached, but insert just in case
                            let mut locations: HashSet<Position, PositionBuildHasher> = HashSet::with_hasher(PositionBuildHasher);
                            locations.insert(neighbor);
                            garden.insert(plant, vec![GardenPlot::new(1, 1, locations)]);
                        }
                    }
                }
            }
        }
    }
}

#[allow(non_snake_case)]
fn part1(contents: String) -> u64 {
    let mut garden: HashMap<char, Vec<GardenPlot>> = HashMap::new();
    let mut map: Vec<Vec<char>> = Vec::new();

    // build map
    for (_col_num, line) in contents.lines().enumerate() {
        let mut garden_row: Vec<char> = Vec::new();
        for (_row_num, c) in line.chars().enumerate() {
            garden_row.push(c);

            if !garden.contains_key(&c) {
                let l: HashSet<Position, PositionBuildHasher> = HashSet::with_hasher(PositionBuildHasher);
                garden.insert(c, vec![GardenPlot::new(0, 0, l)]);
            }
        }

        map.push(garden_row);
    }

    // loop through the map and find all plots
    for (row_num, row) in map.iter().enumerate() {
        for (col_num, plant) in row.iter().enumerate() {
            let plant_loc: Position = Position::new(col_num as i32, row_num as i32);
            let plots: Option<&Vec<GardenPlot>> = garden.get(plant);

            // this plant species has not been discovered or this specific location has not been visited yet
            if plots.is_none() {
                BreadthFirstSearch(&map, plant_loc, &mut garden, *plant);
            } else {
                // is_some
                let mut found_location = false;
                for plot in plots.unwrap() {
                    if plot.locations.contains(&plant_loc) {
                        found_location = true;
                        break;
                    }
                }

                if !found_location {
                    // 'A' region exists, but this specific location within the 'A' has not been found
                    BreadthFirstSearch(&map, plant_loc, &mut garden, *plant);
                }
            }
        }
    }

    let mut answer: u64 = 0;
    for (_, plots) in garden.iter() {
        for plot in plots {
            answer += plot.area * plot.perimeter;
        }
    }
    return answer;
}


#[allow(non_snake_case)]
fn part2(contents: String) -> u64 {
    let mut answer: u64 = 0;

    for (_line_num, line) in contents.lines().enumerate() {

    }

    return answer;
}


#[cfg(test)] #[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test_part1a() {
        let contents: String = fs::read_to_string("src/test1a.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 140);
    }

    #[test]
    fn test_part1b() {
        let contents: String = fs::read_to_string("src/test1b.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 772);
    }

    #[test]
    fn test_part1c() {
        let contents: String = fs::read_to_string("src/test1c.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 1930);
    }

    #[test]
    fn test_part2() {
        let contents: String = fs::read_to_string("src/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), 22);
    }
}