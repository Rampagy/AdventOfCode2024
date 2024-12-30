mod position;

use std::fs;
use std::time::Instant;
use std::collections::HashMap;
use position::Position;

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
    let part2: u64 = part2(contents.clone(), 26);
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

            if (numpad_next_pos.y < 3 && current_position.y < 3) || (numpad_next_pos.x > 0 && current_position.x > 0) {
                // prioritize < over ^ over v over >
                if position_delta.x < 0 {
                    // <
                    for _ in 0..position_delta.x.abs() {
                        numpad_path.push('<');
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

                if position_delta.x > 0 {
                    // >
                    for _ in 0..position_delta.x.abs() {
                        numpad_path.push('>');
                    }
                }
                numpad_path.push('A');
            } else {
                // going to (or from) 0/A, use a different priority to avoid the blank spot
                // hardcode the paths
                if current_position == Position::new(1, 3) { // currently at '0'
                    if numpad_next_char == '1' {
                        // going to '1' from '0' - optimal path is ^<A
                        numpad_path.append(&mut vec!['^','<','A']);
                    } else if numpad_next_char == '4' {
                        // going to '4' from '0' - optimal path is ^^<A
                        numpad_path.append(&mut vec!['^','^','<','A']);
                    } else if numpad_next_char == '7' {
                        // going to '7' from '0' - optimal path is ^^^<A
                        numpad_path.append(&mut vec!['^','^','^','<','A']);
                    } else {
                        panic!("uh oh");
                    }
                } else if current_position == Position::new(0, 2) { // currently at '1'
                    if numpad_next_char == '0' {
                        // going to '0' from '1' - optimal path is >vA
                        numpad_path.append(&mut vec!['>','v','A']);
                    } else if numpad_next_char == 'A' {
                        // going to 'A' from '1' - optimal path is >>vA
                        numpad_path.append(&mut vec!['>','>','v','A']);
                    } else {
                        panic!("uh oh");
                    }
                } else if current_position == Position::new(2, 3) { // currently at 'A'
                    if numpad_next_char == '1' {
                        // going to '1' from 'A' - optimal path is ^<<A
                        numpad_path.append(&mut vec!['^','<','<','A']);
                    } else if numpad_next_char == '4' {
                        // going to '4' from 'A' - optimal path is ^^<<A
                        numpad_path.append(&mut vec!['^','^','<','<','A']);
                    } else if numpad_next_char == '7' {
                        // going to '7' from 'A' - optimal path is ^^^<<A
                        numpad_path.append(&mut vec!['^','^','^','<','<','A']);
                    } else {
                        panic!("uh oh");
                    }
                } else if current_position == Position::new(0, 1) { // currently at '4'
                    if numpad_next_char == '0' {
                        // going to '0' from '4' - optimal path is >vvA
                        numpad_path.append(&mut vec!['>','v','v','A']);
                    } else if numpad_next_char == 'A' {
                        // going to 'A' from '4' - optimal path is >>vvA
                        numpad_path.append(&mut vec!['>','>','v','v','A']);
                    } else {
                        panic!("uh oh");
                    }
                } else if current_position == Position::new(0, 0) { // currently at '7'
                    if numpad_next_char == '0' {
                        // going to '0' from '7' - optimal path is >vvvA
                        numpad_path.append(&mut vec!['>','v','v','v','A']);
                    } else if numpad_next_char == 'A' {
                        // going to 'A' from '7' - optimal path is >>vvvA
                        numpad_path.append(&mut vec!['>','>','v','v','v','A']);
                    } else {
                        panic!("uh oh");
                    }
                } else {
                    panic!("uh oh");
                }
            }

            current_position = numpad_next_pos;
        }

        let mut direction_pads: Vec<Position> = vec![*dirpad_positions_LUT.get(&'A').unwrap(); (num_direction_pads-1) as usize];
        for robot_depth in 0..num_direction_pads-1 {
            let mut new_numpad_path: Vec<char> = Vec::new();
            for ch in numpad_path.clone() {
                let new_pointer_location: Position = *dirpad_positions_LUT.get(&ch).unwrap();
                let current_pointer_location: Position = *direction_pads.get(robot_depth as usize).unwrap();

                // set the new direction pad location
                direction_pads[robot_depth as usize] = new_pointer_location;

                // direction pad is small enough that I'm just hardcoding the optimal path
                if current_pointer_location == *dirpad_positions_LUT.get(&'>').unwrap() && 
                        new_pointer_location == *dirpad_positions_LUT.get(&'^').unwrap() {
                    // from '>' to '^'  :  only possible way is <^A
                    new_numpad_path.append(&mut vec!['<','^','A']);
                } else if current_pointer_location == *dirpad_positions_LUT.get(&'^').unwrap() && 
                        new_pointer_location == *dirpad_positions_LUT.get(&'>').unwrap() {
                    // from '^' to '>'  :  only possible way is v>A
                    new_numpad_path.append(&mut vec!['v','>','A']);
                } else if current_pointer_location == *dirpad_positions_LUT.get(&'A').unwrap() && 
                        new_pointer_location == *dirpad_positions_LUT.get(&'v').unwrap() {
                    // from 'A' to 'v'  :  only possible way is <vA
                    new_numpad_path.append(&mut vec!['<','v','A']);
                } else if current_pointer_location == *dirpad_positions_LUT.get(&'v').unwrap() && 
                        new_pointer_location == *dirpad_positions_LUT.get(&'A').unwrap() {
                    // from 'v' to 'A'  :  only possible way is ^>A
                    new_numpad_path.append(&mut vec!['^','>','A']);
                } else if current_pointer_location == *dirpad_positions_LUT.get(&'A').unwrap() && 
                        new_pointer_location == *dirpad_positions_LUT.get(&'^').unwrap() {
                    // from 'A' to '^'  :  only possible way is <A
                    new_numpad_path.append(&mut vec!['<','A']);
                } else if current_pointer_location == *dirpad_positions_LUT.get(&'^').unwrap() && 
                        new_pointer_location == *dirpad_positions_LUT.get(&'A').unwrap() {
                    // from '^' to 'A'  :  only possible way is >A
                    new_numpad_path.append(&mut vec!['>','A']);
                } else if current_pointer_location == *dirpad_positions_LUT.get(&'v').unwrap() && 
                        new_pointer_location == *dirpad_positions_LUT.get(&'^').unwrap() {
                    // from 'v' to '^'  :  only possible way is ^A
                    new_numpad_path.append(&mut vec!['^','A']);
                } else if current_pointer_location == *dirpad_positions_LUT.get(&'^').unwrap() && 
                        new_pointer_location == *dirpad_positions_LUT.get(&'v').unwrap() {
                    // from '^' to 'v'  :  only possible way is vA
                    new_numpad_path.append(&mut vec!['v','A']);
                } else if current_pointer_location == *dirpad_positions_LUT.get(&'A').unwrap() && 
                        new_pointer_location == *dirpad_positions_LUT.get(&'>').unwrap() {
                    // from 'A' to '>'  :  only possible way is vA
                    new_numpad_path.append(&mut vec!['v','A']);
                } else if current_pointer_location == *dirpad_positions_LUT.get(&'>').unwrap() && 
                        new_pointer_location == *dirpad_positions_LUT.get(&'A').unwrap() {
                    // from '>' to 'A'  :  only possible way is ^A
                    new_numpad_path.append(&mut vec!['^','A']);
                } else if current_pointer_location == *dirpad_positions_LUT.get(&'v').unwrap() && 
                        new_pointer_location == *dirpad_positions_LUT.get(&'>').unwrap() {
                    // from 'v' to '>'  :  only possible way is >A
                    new_numpad_path.append(&mut vec!['>','A']);
                } else if current_pointer_location == *dirpad_positions_LUT.get(&'>').unwrap() && 
                        new_pointer_location == *dirpad_positions_LUT.get(&'v').unwrap() {
                    // from '>' to 'v'  :  only possible way is <A
                    new_numpad_path.append(&mut vec!['<','A']);
                } else if current_pointer_location == *dirpad_positions_LUT.get(&'<').unwrap() && 
                        new_pointer_location == *dirpad_positions_LUT.get(&'v').unwrap() {
                    // from '<' to 'v'  :  only possible way is >A
                    new_numpad_path.append(&mut vec!['>','A']);
                } else if current_pointer_location == *dirpad_positions_LUT.get(&'v').unwrap() && 
                        new_pointer_location == *dirpad_positions_LUT.get(&'<').unwrap() {
                    // from 'v' to '<'  :  only possible way is <A
                    new_numpad_path.append(&mut vec!['<','A']);
                } else if current_pointer_location == *dirpad_positions_LUT.get(&'<').unwrap() && 
                        new_pointer_location == *dirpad_positions_LUT.get(&'>').unwrap() {
                    // from '<' to '>'  :  only possible way is >>A
                    new_numpad_path.append(&mut vec!['>','>','A']);
                } else if current_pointer_location == *dirpad_positions_LUT.get(&'>').unwrap() && 
                        new_pointer_location == *dirpad_positions_LUT.get(&'<').unwrap() {
                    // from '>' to '<'  :  only possible way is <<A
                    new_numpad_path.append(&mut vec!['<','<','A']);
                } else if current_pointer_location == *dirpad_positions_LUT.get(&'<').unwrap() && 
                        new_pointer_location == *dirpad_positions_LUT.get(&'^').unwrap() {
                    // from '<' to '^'  :  only possible way is >^A
                    new_numpad_path.append(&mut vec!['>','^','A']);
                } else if current_pointer_location == *dirpad_positions_LUT.get(&'^').unwrap() && 
                        new_pointer_location == *dirpad_positions_LUT.get(&'<').unwrap() {
                    // from '^' to '<'  :  only possible way is v<A
                    new_numpad_path.append(&mut vec!['v','<','A']);
                } else if current_pointer_location == *dirpad_positions_LUT.get(&'<').unwrap() && 
                        new_pointer_location == *dirpad_positions_LUT.get(&'A').unwrap() {
                    // from '<' to 'A'  :  only possible way is >>^A
                    new_numpad_path.append(&mut vec!['>','>','^','A']);
                } else if current_pointer_location == *dirpad_positions_LUT.get(&'A').unwrap() && 
                        new_pointer_location == *dirpad_positions_LUT.get(&'<').unwrap() {
                    // from 'A' to '<'  :  only possible way is v<<A
                    new_numpad_path.append(&mut vec!['v','<','<','A']);
                } else if current_pointer_location == new_pointer_location {
                    // if new and previous are the same only insert an 'A'
                    new_numpad_path.push('A');
                } else {
                    panic!("uh oh");
                }
            }

            // override the existing numpad_path
            numpad_path = new_numpad_path;
        }

        let numeric_part_of_code: u64 = seq.iter().collect::<String>()
                                            .trim_end_matches('A')
                                            .parse::<u64>().unwrap();
        answer += numpad_path.len() as u64 * numeric_part_of_code;
    }

    return answer;
}


#[allow(non_snake_case)]
fn part2(contents: String, num_direction_pads: u64) -> u64 {
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

            if (numpad_next_pos.y < 3 && current_position.y < 3) || (numpad_next_pos.x > 0 && current_position.x > 0) {
                // prioritize < over ^ over v over >
                if position_delta.x < 0 {
                    // <
                    for _ in 0..position_delta.x.abs() {
                        numpad_path.push('<');
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

                if position_delta.x > 0 {
                    // >
                    for _ in 0..position_delta.x.abs() {
                        numpad_path.push('>');
                    }
                }
                numpad_path.push('A');
            } else {
                // going to (or from) 0/A, use a different priority to avoid the blank spot
                // hardcode the paths
                if current_position == Position::new(1, 3) { // currently at '0'
                    if numpad_next_char == '1' {
                        // going to '1' from '0' - optimal path is ^<A
                        numpad_path.append(&mut vec!['^','<','A']);
                    } else if numpad_next_char == '4' {
                        // going to '4' from '0' - optimal path is ^^<A
                        numpad_path.append(&mut vec!['^','^','<','A']);
                    } else if numpad_next_char == '7' {
                        // going to '7' from '0' - optimal path is ^^^<A
                        numpad_path.append(&mut vec!['^','^','^','<','A']);
                    } else {
                        panic!("uh oh");
                    }
                } else if current_position == Position::new(0, 2) { // currently at '1'
                    if numpad_next_char == '0' {
                        // going to '0' from '1' - optimal path is >vA
                        numpad_path.append(&mut vec!['>','v','A']);
                    } else if numpad_next_char == 'A' {
                        // going to 'A' from '1' - optimal path is >>vA
                        numpad_path.append(&mut vec!['>','>','v','A']);
                    } else {
                        panic!("uh oh");
                    }
                } else if current_position == Position::new(2, 3) { // currently at 'A'
                    if numpad_next_char == '1' {
                        // going to '1' from 'A' - optimal path is ^<<A
                        numpad_path.append(&mut vec!['^','<','<','A']);
                    } else if numpad_next_char == '4' {
                        // going to '4' from 'A' - optimal path is ^^<<A
                        numpad_path.append(&mut vec!['^','^','<','<','A']);
                    } else if numpad_next_char == '7' {
                        // going to '7' from 'A' - optimal path is ^^^<<A
                        numpad_path.append(&mut vec!['^','^','^','<','<','A']);
                    } else {
                        panic!("uh oh");
                    }
                } else if current_position == Position::new(0, 1) { // currently at '4'
                    if numpad_next_char == '0' {
                        // going to '0' from '4' - optimal path is >vvA
                        numpad_path.append(&mut vec!['>','v','v','A']);
                    } else if numpad_next_char == 'A' {
                        // going to 'A' from '4' - optimal path is >>vvA
                        numpad_path.append(&mut vec!['>','>','v','v','A']);
                    } else {
                        panic!("uh oh");
                    }
                } else if current_position == Position::new(0, 0) { // currently at '7'
                    if numpad_next_char == '0' {
                        // going to '0' from '7' - optimal path is >vvvA
                        numpad_path.append(&mut vec!['>','v','v','v','A']);
                    } else if numpad_next_char == 'A' {
                        // going to 'A' from '7' - optimal path is >>vvvA
                        numpad_path.append(&mut vec!['>','>','v','v','v','A']);
                    } else {
                        panic!("uh oh");
                    }
                } else {
                    panic!("uh oh");
                }
            }

            current_position = numpad_next_pos;
        }

        // do a recursive search with depth to find the length of the sequence
        let mut count: u64 = 0;
        recursive_search(numpad_path, num_direction_pads, &mut count, &dirpad_positions_LUT);
        let numeric_part_of_code: u64 = seq.iter().collect::<String>()
                                            .trim_end_matches('A')
                                            .parse::<u64>().unwrap();
        answer += count * numeric_part_of_code;
    }

    return answer;
}

#[allow(non_snake_case)]
fn recursive_search(sequence: Vec<char>, depth: u64, count: &mut u64, dirpad_positions_LUT: &HashMap<char, Position>) -> () {
    // find the locations of the 'A' in subsequence
    let mut pos_a: Vec<usize> = vec![0];
    for (i, ch) in sequence.clone().iter().enumerate() {
        if *ch == 'A' {
            pos_a.push(i+1);
        }
    }

    for i in 0..pos_a.len()-1 {
        let mut subsequence_vec: Vec<char> = Vec::new();
        for j in pos_a[i]..pos_a[i+1] {
            subsequence_vec.push(sequence[j]);
        }

        // generate the new depth-1 sequence
        let new_subsequence: Vec<char> = get_new_sequence(subsequence_vec.clone(), dirpad_positions_LUT);

        if depth-1 > 0 {
            // search this subsequence for count
            recursive_search(new_subsequence, depth-1, count, dirpad_positions_LUT);
        } else {
            // recursive search is done, increment count
            *count += subsequence_vec.len() as u64;
        }
    }

    return;
}

#[allow(non_snake_case)]
fn get_new_sequence(seq: Vec<char>, dirpad_positions_LUT: &HashMap<char, Position>) -> Vec<char> {
    let mut new_seq: Vec<char> = Vec::new();
    let mut current_pointer_location: Position = Position::new(2, 0); // always starts at 'A'?

    for ch in seq.clone() {
        let new_pointer_location: Position = *dirpad_positions_LUT.get(&ch).unwrap();

        // direction pad is small enough that I'm just hardcoding the optimal path
        if current_pointer_location == *dirpad_positions_LUT.get(&'>').unwrap() && 
                new_pointer_location == *dirpad_positions_LUT.get(&'^').unwrap() {
            // from '>' to '^'  :  only possible way is <^A
            new_seq.append(&mut vec!['<','^','A']);
        } else if current_pointer_location == *dirpad_positions_LUT.get(&'^').unwrap() && 
                new_pointer_location == *dirpad_positions_LUT.get(&'>').unwrap() {
            // from '^' to '>'  :  only possible way is v>A
            new_seq.append(&mut vec!['v','>','A']);
        } else if current_pointer_location == *dirpad_positions_LUT.get(&'A').unwrap() && 
                new_pointer_location == *dirpad_positions_LUT.get(&'v').unwrap() {
            // from 'A' to 'v'  :  only possible way is <vA
            new_seq.append(&mut vec!['<','v','A']);
        } else if current_pointer_location == *dirpad_positions_LUT.get(&'v').unwrap() && 
                new_pointer_location == *dirpad_positions_LUT.get(&'A').unwrap() {
            // from 'v' to 'A'  :  only possible way is ^>A
            new_seq.append(&mut vec!['^','>','A']);
        } else if current_pointer_location == *dirpad_positions_LUT.get(&'A').unwrap() && 
                new_pointer_location == *dirpad_positions_LUT.get(&'^').unwrap() {
            // from 'A' to '^'  :  only possible way is <A
            new_seq.append(&mut vec!['<','A']);
        } else if current_pointer_location == *dirpad_positions_LUT.get(&'^').unwrap() && 
                new_pointer_location == *dirpad_positions_LUT.get(&'A').unwrap() {
            // from '^' to 'A'  :  only possible way is >A
            new_seq.append(&mut vec!['>','A']);
        } else if current_pointer_location == *dirpad_positions_LUT.get(&'v').unwrap() && 
                new_pointer_location == *dirpad_positions_LUT.get(&'^').unwrap() {
            // from 'v' to '^'  :  only possible way is ^A
            new_seq.append(&mut vec!['^','A']);
        } else if current_pointer_location == *dirpad_positions_LUT.get(&'^').unwrap() && 
                new_pointer_location == *dirpad_positions_LUT.get(&'v').unwrap() {
            // from '^' to 'v'  :  only possible way is vA
            new_seq.append(&mut vec!['v','A']);
        } else if current_pointer_location == *dirpad_positions_LUT.get(&'A').unwrap() && 
                new_pointer_location == *dirpad_positions_LUT.get(&'>').unwrap() {
            // from 'A' to '>'  :  only possible way is vA
            new_seq.append(&mut vec!['v','A']);
        } else if current_pointer_location == *dirpad_positions_LUT.get(&'>').unwrap() && 
                new_pointer_location == *dirpad_positions_LUT.get(&'A').unwrap() {
            // from '>' to 'A'  :  only possible way is ^A
            new_seq.append(&mut vec!['^','A']);
        } else if current_pointer_location == *dirpad_positions_LUT.get(&'v').unwrap() && 
                new_pointer_location == *dirpad_positions_LUT.get(&'>').unwrap() {
            // from 'v' to '>'  :  only possible way is >A
            new_seq.append(&mut vec!['>','A']);
        } else if current_pointer_location == *dirpad_positions_LUT.get(&'>').unwrap() && 
                new_pointer_location == *dirpad_positions_LUT.get(&'v').unwrap() {
            // from '>' to 'v'  :  only possible way is <A
            new_seq.append(&mut vec!['<','A']);
        } else if current_pointer_location == *dirpad_positions_LUT.get(&'<').unwrap() && 
                new_pointer_location == *dirpad_positions_LUT.get(&'v').unwrap() {
            // from '<' to 'v'  :  only possible way is >A
            new_seq.append(&mut vec!['>','A']);
        } else if current_pointer_location == *dirpad_positions_LUT.get(&'v').unwrap() && 
                new_pointer_location == *dirpad_positions_LUT.get(&'<').unwrap() {
            // from 'v' to '<'  :  only possible way is <A
            new_seq.append(&mut vec!['<','A']);
        } else if current_pointer_location == *dirpad_positions_LUT.get(&'<').unwrap() && 
                new_pointer_location == *dirpad_positions_LUT.get(&'>').unwrap() {
            // from '<' to '>'  :  only possible way is >>A
            new_seq.append(&mut vec!['>','>','A']);
        } else if current_pointer_location == *dirpad_positions_LUT.get(&'>').unwrap() && 
                new_pointer_location == *dirpad_positions_LUT.get(&'<').unwrap() {
            // from '>' to '<'  :  only possible way is <<A
            new_seq.append(&mut vec!['<','<','A']);
        } else if current_pointer_location == *dirpad_positions_LUT.get(&'<').unwrap() && 
                new_pointer_location == *dirpad_positions_LUT.get(&'^').unwrap() {
            // from '<' to '^'  :  only possible way is >^A
            new_seq.append(&mut vec!['>','^','A']);
        } else if current_pointer_location == *dirpad_positions_LUT.get(&'^').unwrap() && 
                new_pointer_location == *dirpad_positions_LUT.get(&'<').unwrap() {
            // from '^' to '<'  :  only possible way is v<A
            new_seq.append(&mut vec!['v','<','A']);
        } else if current_pointer_location == *dirpad_positions_LUT.get(&'<').unwrap() && 
                new_pointer_location == *dirpad_positions_LUT.get(&'A').unwrap() {
            // from '<' to 'A'  :  only possible way is >>^A
            new_seq.append(&mut vec!['>','>','^','A']);
        } else if current_pointer_location == *dirpad_positions_LUT.get(&'A').unwrap() && 
                new_pointer_location == *dirpad_positions_LUT.get(&'<').unwrap() {
            // from 'A' to '<'  :  only possible way is v<<A
            new_seq.append(&mut vec!['v','<','<','A']);
        } else if current_pointer_location == new_pointer_location {
            // if new and previous are the same only insert an 'A'
            new_seq.push('A');
        } else {
            panic!("uh oh");
        }

        current_pointer_location = new_pointer_location;
    }

    return new_seq;
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
        assert_eq!(part2(contents.clone(), 3), 126384);
    }
}