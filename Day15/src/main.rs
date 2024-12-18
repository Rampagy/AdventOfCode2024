mod position;

use std::collections::HashSet;
use std::fs;
use std::time::Instant;
use position::{Position, PositionBuildHasher};
use std::collections::VecDeque;


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

fn valid_robot_position(robot: Position, robot_offset: Position, 
            boxes:  HashSet<Position, PositionBuildHasher>, 
            walls:  HashSet<Position, PositionBuildHasher>) -> Option<Position> {
    // go until a wall or empty space
    let new_robot: Position = robot + robot_offset;
    if walls.contains(&new_robot) {
        // wall
        return None;
    } else if boxes.contains(&new_robot) {
        // check if movable box
        return valid_robot_position(new_robot, robot_offset, boxes, walls);
    } else {
        // empty space
        return Some(new_robot);
    }
}

#[allow(non_snake_case)]
fn part1(contents: String) -> u64 {
    let mut robot: Position = Position::new(0, 0);
    let mut boxes: HashSet<Position, PositionBuildHasher> = HashSet::with_hasher(PositionBuildHasher);
    let mut walls: HashSet<Position, PositionBuildHasher> = HashSet::with_hasher(PositionBuildHasher);

    let mut capture_map: bool = true;
    for (row_num, line) in contents.lines().enumerate() {
        if line == "" {
            capture_map = false;
        } else {
            if capture_map {
                for (col_num, c) in line.chars().enumerate() {
                    if c == '#' {
                        walls.insert(Position::new(col_num as i32, row_num as i32));
                    } else if c == 'O' {
                        boxes.insert(Position::new(col_num as i32, row_num as i32));
                    } else if c == '@' {
                        robot = Position::new(col_num as i32, row_num as i32);
                    }
                }
            } else {
                for direction in line.chars() {
                    let robot_offset: Position = 
                            if direction == '^' { Position::new(0, -1) } //north
                            else if direction == '>' { Position::new(1, 0) } // east
                            else if direction == 'v' { Position::new(0, 1) } // south
                            else if direction == '<' { Position::new(-1, 0) }  // west
                            else { Position::new(0,0) };
            
                    let empty_location: Option<Position> = valid_robot_position(robot, robot_offset, boxes.clone(), walls.clone());
                    if empty_location.is_some() {
                        // basically this is swapping the rock that gets displaced with the empty location
            
                        // calculate new robot position
                        robot = robot + robot_offset;
            
                        // remove box at robots new location
                        boxes.remove(&robot);
            
                        // insert box at new location if it is not the robot
                        if empty_location.unwrap() != robot {
                            boxes.insert(empty_location.unwrap());
                        }
                    }
                }
            }
        }
    }

    let mut answer: u64 = 0;
    for b in boxes {
        answer += 100 * b.y as u64 + b.x as u64;
    }
    return answer;
}

#[allow(non_snake_case)]
fn part2(contents: String) -> u64 {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut direction: Vec<char> = Vec::new();
    let mut robot: Position = Position::new(0, 0);

    let mut capture_map: bool = true;
    for (row_num, line) in contents.lines().enumerate() {
        if line == "" {
            capture_map = false;
        } else {
            if capture_map {
                let mut map_row: Vec<char> = Vec::new();
                for (col_num, c) in line.chars().enumerate() {
                    if c == '#' {
                        map_row.push('#');
                        map_row.push('#');
                    } else if c == 'O' {
                        map_row.push('[');
                        map_row.push(']');
                    } else if c == '@' {
                        robot = Position::new(2*col_num as i32, row_num as i32);
                        map_row.push('@');
                        map_row.push('.');
                    } else {
                        map_row.push('.');
                        map_row.push('.');
                    }
                }
                map.push(map_row);
            } else {
                for c in line.chars() {
                    direction.push(c);
                }
            }
        }
    }

    for dir in direction {
        let robot_offset: Position = 
                if dir == '^' { Position::new(0, -1) } //north
                else if dir == '>' { Position::new(1, 0) } // east
                else if dir == 'v' { Position::new(0, 1) } // south
                else if dir == '<' { Position::new(-1, 0) }  // west
                else { Position::new(0,0) };

        let mut rock_queue: VecDeque<Position> = VecDeque::from([robot + robot_offset]);
        let mut temp_map: Vec<Vec<char>> = map.clone();
        let mut moved_locations: HashSet<Position, PositionBuildHasher> = HashSet::with_hasher(PositionBuildHasher);

        // move robot to new location
        temp_map[robot.y as usize][robot.x as usize] = '.';
        temp_map[(robot.y + robot_offset.y) as usize][(robot.x + robot_offset.x) as usize] = '@';
        moved_locations.insert(robot);

        let mut successful_move: bool = true;
        while !rock_queue.is_empty() {
            // get current and see if it can move
            let current: Position = rock_queue.pop_front().unwrap();

            if map[current.y as usize][current.x as usize] == '#' {
                // wall - no changes
                successful_move = false;
                break;
            } else if map[current.y as usize][current.x as usize] == '[' {
                // left side of a rock
                // add new position to rock_queue and update the temp_map
                rock_queue.push_back(current + robot_offset);
                temp_map[(current.y+robot_offset.y) as usize][(current.x+robot_offset.x) as usize] = '[';
                moved_locations.insert(current);

                // check the opposite direction of the move to see what should be put in current's old location
                if !moved_locations.contains(&(current + Position::new(0, -robot_offset.y))) {
                    temp_map[current.y as usize][current.x as usize] = '.';
                }

                // if direction is north or south additionally add the other side of the rock to the rock_queue and update temp map
                if robot_offset == Position::new(0, -1) || robot_offset == Position::new(0, 1) {
                    temp_map[(current.y + robot_offset.y) as usize][(current.x + robot_offset.x + 1) as usize] = ']';
                    rock_queue.push_back(current + robot_offset + Position::new(1, 0));
                    moved_locations.insert(current + Position::new(1, 0));

                    // check the opposite direction of the move to see what should be put in current's old location
                    if !moved_locations.contains(&(current + Position::new(1, -robot_offset.y))) {
                        temp_map[current.y as usize][(current.x + 1) as usize] = '.';
                    }
                }
            } else if map[current.y as usize][current.x as usize] == ']' {
                // right side of a rock
                // add new position to rock_queue and update the temp_map
                rock_queue.push_back(current + robot_offset);
                temp_map[(current.y+robot_offset.y) as usize][(current.x+robot_offset.x) as usize] = ']';
                moved_locations.insert(current);

                // check the opposite direction of the move to see what should be put in current's old location
                if !moved_locations.contains(&(current + Position::new(0, -robot_offset.y))) {
                    temp_map[current.y as usize][current.x as usize] = '.';
                }

                // if direction is north or south additionally add the other side of the rock to the rock_queue and update temp map
                if robot_offset == Position::new(0, -1) || robot_offset == Position::new(0, 1) {
                    temp_map[(current.y + robot_offset.y) as usize][(current.x + robot_offset.x - 1) as usize] = '[';
                    rock_queue.push_back(current + robot_offset + Position::new(-1, 0));
                    moved_locations.insert(current + Position::new(-1, 0));

                    // check the opposite direction of the move to see what should be put in current's old location
                    if !moved_locations.contains(&(current + Position::new(-1, -robot_offset.y))) {
                        temp_map[current.y as usize][(current.x - 1) as usize] = '.';
                    }
                }
            } else {
                // empty location - no changes
            }
        }

        if successful_move {
            map = temp_map;
            robot = robot + robot_offset;
        }

        /*
        // print map
        for row in map.clone() {
            for character in row {
                print!("{}", character);
            }
            println!();
        }
        println!();
        */
    }

    let mut answer: u64 = 0;
    for (row_num, row) in map.iter().enumerate() {
        for (col_num, char) in row.iter().enumerate() {
            if *char == '[' {
                answer += 100 * row_num as u64 + col_num as u64;
            }
        }
    }
    return answer;
}


#[cfg(test)] #[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test_part1a() {
        let contents: String = fs::read_to_string("src/test1a.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 2028);
    }

    #[test]
    fn test_part1b() {
        let contents: String = fs::read_to_string("src/test1b.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 10092);
    }

    #[test]
    fn test_part2a() {
        let contents: String = fs::read_to_string("src/test2a.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), 618);
    }

    #[test]
    fn test_part2b() {
        let contents: String = fs::read_to_string("src/test2b.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), 9021);
    }
}