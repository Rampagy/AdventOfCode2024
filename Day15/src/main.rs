mod position;

use std::collections::HashSet;
use std::fs;
use std::time::Instant;
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

    println!("part 2: {} ({:.2?})", part2, elapsed); // 777 too low, 778 too low
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
    let mut direction: Vec<char> = Vec::new();

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

    let mut answer: u64 = 0;
    for b in boxes {
        answer += 100 * b.y as u64 + b.x as u64;
    }
    return answer;
}



#[allow(non_snake_case)]
fn part2(contents: String) -> u64 {
    let mut robots: Vec<(Position, Position)> = Vec::new();

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
        assert_eq!(part1(contents.clone()), 2028);
    }

    #[test]
    fn test_part1b() {
        let contents: String = fs::read_to_string("src/test1b.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 10092);
    }

    #[test]
    fn test_part2() {
        let contents: String = fs::read_to_string("src/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), 12);
    }
}