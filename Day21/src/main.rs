mod position;

use std::fs;
use std::time::Instant;
use std::collections::{HashMap, HashSet};
use position::{Position, PositionBuildHasher};

#[allow(non_snake_case)]
fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");
    let mut now: Instant;
    let mut elapsed: std::time::Duration;

    now = Instant::now();
    let part1: u64 = part1(contents.clone(), 3);
    elapsed = now.elapsed();

    println!("part 1: {} ({:.2?})", part1, elapsed);

    now = Instant::now();
    let part2: u64 = part2(contents.clone());
    elapsed = now.elapsed();

    println!("part 2: {} ({:.2?})", part2, elapsed);
}


#[allow(non_snake_case)]
fn part1(contents: String, num_direction_pads: u64) -> u64 {
    let mut answer: u64 = 0;
    let mut sequences: Vec<Vec<char>> = Vec::new();

    for (_row_num, line) in contents.lines().enumerate() {
        let mut row: Vec<char> = Vec::new();
        for ch in line.chars() {
            row.push(ch);
        }

        sequences.push(row);
    }

    let numpad_positions: [Position; 11] = [ // 0,0 is top left
        Position::new(1, 3), // 0
        Position::new(0, 2), // 1
        Position::new(1, 2), // 2
        Position::new(2, 2), // 3
        Position::new(0, 1), // 4
        Position::new(1, 1), // 5
        Position::new(2, 1), // 6
        Position::new(0, 0), // 7
        Position::new(1, 0), // 8
        Position::new(2, 0), // 9
        Position::new(2, 3), // A
    ];

    let mut dirpad_positions_LUT: HashMap<char, Position> = HashMap::new(); // 0,0 is top left
    dirpad_positions_LUT.insert('A', Position::new(2, 0));
    dirpad_positions_LUT.insert('^', Position::new(1, 0));
    dirpad_positions_LUT.insert('>', Position::new(2, 1));
    dirpad_positions_LUT.insert('v', Position::new(1, 1));
    dirpad_positions_LUT.insert('<', Position::new(0, 1));


    for seq in sequences {
        let mut current_position: Position = Position::new(2, 3);
        let mut numpad_path: Vec<char> = Vec::new();

        for i in 0..seq.len() {
            let numpad_next_char: char = *seq.get(i).unwrap();
            let numpad_next_pos: Position = *numpad_positions.get(numpad_next_char.to_digit(10).unwrap_or(10) as usize).unwrap();
            let position_delta: Position = numpad_next_pos - current_position;

            // prioritize > over ^ over v over <
            if position_delta.x > 0 {
                // >
                for _ in 0..position_delta.x.abs() {
                    numpad_path.push('>');
                }
            }

            if position_delta.y < 0 {
                // ^
                for _ in 0..position_delta.y.abs() {
                    numpad_path.push('^');
                }
            }


            if position_delta.y > 0 {
                // v
                for _ in 0..position_delta.y.abs() {
                    numpad_path.push('v');
                }
            }

            if position_delta.x < 0 {
                // <
                for _ in 0..position_delta.x.abs() {
                    numpad_path.push('<');
                }
            }






            numpad_path.push('A');
            current_position = numpad_next_pos;
        }

        let mut out: String = String::new();
        for ch in numpad_path.clone() {
            out += ch.to_string().as_str();
        }

        println!("{}", out);

        // TODO: direction pads
        let mut direction_pads: Vec<Position> = vec![*dirpad_positions_LUT.get(&'A').unwrap(); (num_direction_pads-1) as usize];
        for robot_depth in 0..num_direction_pads-1 {
            let mut new_numpad_path: Vec<char> = Vec::new();
            for ch in numpad_path.clone() {
                let new_pointer_location: Position = *dirpad_positions_LUT.get(&ch).unwrap();
                let delta_pointer_location: Position = new_pointer_location - *direction_pads.get(robot_depth as usize).unwrap();

                // set the new direction pad location
                direction_pads[robot_depth as usize] = new_pointer_location;

                // prioritize > over v over ^ over <
                if delta_pointer_location.x > 0 {
                    // >
                    for _ in 0..delta_pointer_location.x.abs() {
                        new_numpad_path.push('>');
                    }
                }

                if delta_pointer_location.y > 0 {
                    // v
                    for _ in 0..delta_pointer_location.y.abs() {
                        new_numpad_path.push('v');
                    }
                }

                if delta_pointer_location.y < 0 {
                    // ^
                    for _ in 0..delta_pointer_location.y.abs() {
                        new_numpad_path.push('^');
                    }
                }

                if delta_pointer_location.x < 0 {
                    // <
                    for _ in 0..delta_pointer_location.x.abs() {
                        new_numpad_path.push('<');
                    }
                }

                // activate after every arrow
                new_numpad_path.push('A');
            }

            // override the existing numpad_path
            numpad_path = new_numpad_path;

            let mut out: String = String::new();
            for ch in numpad_path.clone() {
                out += ch.to_string().as_str();
            }

            println!("{}", out);
        }

        let numeric_part_of_code: u64 = seq.iter().collect::<String>()
                                            .trim_end_matches('A')
                                            .parse::<u64>().unwrap();
        answer += numpad_path.len() as u64 * numeric_part_of_code;
        println!("{}: {} * {}", seq.iter().collect::<String>(), numpad_path.len() as u64, numeric_part_of_code);
    }

    return answer;
}



#[allow(non_snake_case)]
fn part2(contents: String) -> u64 {

    for (_row_num, line) in contents.lines().enumerate() {
        // TODO
    }



    return 0;
}



#[cfg(test)] #[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test_part1a() {
        let contents: String = fs::read_to_string("src/test1a.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone(), 3), 126384);
    }

    #[test]
    fn test_part2a() {
        let contents: String = fs::read_to_string("src/test2a.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), 193);
    }

}