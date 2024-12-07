mod position;

use std::fs;
use std::collections::HashSet;
//use std::collections::HashMap;
use position::Position;


#[allow(non_snake_case)]
fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    println!("part 1: {}", part1(contents.clone()));
    println!("part 2: {}", part2(contents.clone()));  // 1797 - too high, 1796 - too high, 1724 - too high, 1680?? , 1663
}

#[allow(non_snake_case)]
fn turn_right(p: Position) -> Position {
    let new_offset: Position = 
    if p.x == 0 && p.y == -1 { // north
        // east
        Position::new(1, 0)
    } else if p.x == 1 && p.y == 0 { // east
        // south
        Position::new(0, 1)
    } else if p.x == 0 && p.y == 1 { // south
        // west
        Position::new(-1, 0)
    } else if p.x == -1 && p.y == 0 { // west
        // north
        Position::new(0, -1)
    } else {
        // shouldn't get here
        Position::new(0, 0)
    };

    return new_offset;
}


#[allow(non_snake_case)]
fn part1(contents: String) -> u64 {
    let mut answer: u64 = 0;
    let mut lab_map: HashSet<Position> = HashSet::new();
    let mut visited_squares: HashSet<Position> = HashSet::new();
    let mut lab_guard_position: Position = Position::new(0, 0);
    let mut map_width: usize = 0;
    let mut map_height: usize = 0;

    for (row_num, line) in contents.lines().enumerate() {
        for (col_num, c) in line.chars().enumerate() {
            if c == '#' {
                lab_map.insert(Position {x: col_num as i32, y: row_num as i32});
            } else if c == '^' {
                lab_guard_position = Position {x: col_num as i32, y: row_num as i32};
            }

            if col_num > map_width {
                map_width = col_num;
            }
        }

        if map_width > map_height {
            map_height = map_width;
        }
    }

    let mut direction_offset: Position = Position::new(0, -1);
    visited_squares.insert(lab_guard_position);
    while lab_guard_position.x >= 0 && lab_guard_position.y >= 0 && 
          lab_guard_position.x <= map_width as i32 && lab_guard_position.y <= map_height as i32 {
            
            let new_lab_guard_position: Position = Position {
                x: lab_guard_position.x + direction_offset.x , 
                y: lab_guard_position.y + direction_offset.y
            };

            // check if the lab_guard_position's next position is #
            if lab_map.contains(&new_lab_guard_position) {
                // we hit an obstacle, turn right
                direction_offset = turn_right(direction_offset);
            } else {
                // move forward
                lab_guard_position = new_lab_guard_position;
                
                if !visited_squares.contains(&lab_guard_position) {
                    visited_squares.insert(lab_guard_position);
                    answer += 1;
                }
            }
    }

    return answer;
}


#[allow(non_snake_case)]
fn part2(contents: String) -> u64 {
    let mut answer: u64 = 0;
    let mut lab_map: HashSet<Position> = HashSet::new();
    let mut lab_guard_position: Position = Position::new(0, 0);
    let mut map_width: usize = 0;
    let mut map_height: usize = 0;

    for (row_num, line) in contents.lines().enumerate() {
        for (col_num, c) in line.chars().enumerate() {
            if c == '#' {
                lab_map.insert(Position {x: col_num as i32, y: row_num as i32});
            } else if c == '^' {
                lab_guard_position = Position {x: col_num as i32, y: row_num as i32};
            }

            if col_num > map_width {
                map_width = col_num;
            }
        }

        if map_width > map_height {
            map_height = map_width;
        }
    }

    let mut direction_offset: Position;
    let lab_guard_init: Position = lab_guard_position;
    for x in 0..(map_width+1) {
        for y in 0..(map_height+1) {
            let new_ob: Position = Position::new(x as i32, y as i32);
            if !lab_map.contains(&new_ob) && lab_guard_init != new_ob {

                // reset lab gaurd position and direction offset
                lab_guard_position = lab_guard_init;
                direction_offset = Position::new(0, -1);

                // make a copy with new object and use that instead
                let mut lab_map_copy: HashSet<Position> = lab_map.clone();
                lab_map_copy.insert(new_ob);

                let mut loop_count: usize = 0;
                while lab_guard_position.x >= 0 && lab_guard_position.y >= 0 && 
                        lab_guard_position.x <= map_width as i32 && lab_guard_position.y <= map_height as i32 {
                    
                    let new_lab_guard_position: Position = Position {
                        x: lab_guard_position.x + direction_offset.x , 
                        y: lab_guard_position.y + direction_offset.y
                    };

                    // check if the lab_guard_position's next position is #
                    if lab_map_copy.contains(&new_lab_guard_position) {
                        // we hit an obstacle, turn right
                        direction_offset = turn_right(direction_offset);
                    } else {
                        // move forward
                        lab_guard_position = new_lab_guard_position;
                    }

                    if loop_count >= 100_000 {
                        // in a loop
                        answer += 1;
                        break;
                    }

                    loop_count = loop_count.saturating_add(1);
                }
            }
        }
    }

    return answer;
}


#[cfg(test)] #[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 41);
    }

    #[test]
    fn test_part2() {
        let contents: String = fs::read_to_string("src/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), 6);
    }
}