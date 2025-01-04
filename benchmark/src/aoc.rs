use super::position::{Position, PositionBuildHasher};

use std::u64;
use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::Ordering;
use itertools::Itertools;
use ordered_float::OrderedFloat;
use priority_queue::PriorityQueue;

#[allow(non_snake_case)]
pub fn d01_part1(contents: String) -> String {
    let mut answer: u64 = 0;
    let mut left_side: Vec<u64> = vec![];
    let mut right_side: Vec<u64> = vec![];

    for line in contents.lines() {
        let both_sides: Vec<u64> = line.to_string()
            .split_ascii_whitespace()
            .map(|x: &str| x.parse::<u64>().unwrap())
            .collect();

        left_side.push(both_sides[0]);
        right_side.push(both_sides[1]);
    }

    left_side.sort_by(|a: &u64, b: &u64| b.cmp(a));
    right_side.sort_by(|a: &u64, b: &u64| b.cmp(a));

    
    for i in 0..left_side.len() {
        answer += (left_side[i] as i64 - right_side[i] as i64).abs() as u64
    }

    return format!("{}", answer);
}


#[allow(non_snake_case)]
pub fn d01_part2(contents: String) -> String {
    let mut answer: u64 = 0;
    let mut left_side: Vec<u64> = vec![];
    let mut right_side: HashMap<u64, u64> = HashMap::new();

    for line in contents.lines() {
        let both_sides: Vec<u64> = line.to_string()
            .split_ascii_whitespace()
            .map(|x: &str| x.parse::<u64>().unwrap())
            .collect();

        left_side.push(both_sides[0]);

        match right_side.get(&both_sides[1]) {
            Some(x) => right_side.insert(both_sides[1], x+1),
            None => right_side.insert(both_sides[1], 1)
        };
    }

    for val in left_side.clone() {
        match right_side.get(&val) {
            Some(x) => answer += x*val,
            None => ()
        }
    }

    return format!("{}", answer);
}

#[allow(non_snake_case)]
pub fn d02_part1(contents: String) -> String {
    let mut answer: u64 = 0;

    for line in contents.lines() {
        let report: Vec<u64> = line.to_string()
            .split_ascii_whitespace()
            .map(|x: &str| x.parse::<u64>().unwrap())
            .collect();

        let mut level_delta: i64 = 0;
        for i in 1..report.len() {
            let delta: i64 = report[i] as i64 - report[i-1] as i64;
            if delta > 0 && delta < 4 {
                level_delta += 1;
            } else if delta < 0 && delta > -4 {
                level_delta -= 1;
            }
        }

        if level_delta.abs() == (report.len()-1) as i64 {
            answer += 1;
        }
    }

    return format!("{}", answer);
}

#[allow(non_snake_case)]
fn day02_check_good(report: Vec<u64>) -> bool {
    let mut level_delta: i64 = 0;
    for i in 1..report.len() {
        let delta: i64 = report[i] as i64 - report[i-1] as i64;
        if delta > 0 && delta < 4 {
            level_delta += 1;
        } else if delta < 0 && delta > -4 {
            level_delta -= 1;
        }
    }

    if level_delta.abs() as usize == report.len()-1 {
        return true;
    } else {
        return false;
    }
}

#[allow(non_snake_case)]
pub fn d02_part2(contents: String) -> String {
    let mut answer: u64 = 0;

    for line in contents.lines() {
        let report: Vec<u64> = line.to_string()
            .split_ascii_whitespace()
            .map(|x: &str| x.parse::<u64>().unwrap())
            .collect();

        if day02_check_good(report.clone()) {
            answer += 1;
        } else {
            for i in 0..report.len() {
                let mut new_report: Vec<u64> = report.clone();
                new_report.remove(i);
                if day02_check_good(new_report) {
                    answer += 1;
                    break;
                }
            }
        }
    }

    return format!("{}", answer);
}


#[allow(non_snake_case)]
pub fn d03_part1(contents: String) -> String {
    let mut answer: u64 = 0;

    let mut pattern_index: u64 = 0;
    let mut num_count: u64 = 0;
    let mut first_num: String = "".to_string();
    let mut second_num: String = "".to_string();

    for (_line_num, line) in contents.lines().enumerate() {
        for letter in line.chars() {
            if letter == 'm' && pattern_index == 0 {
                pattern_index += 1;
            } else if letter == 'u' && pattern_index == 1 {
                pattern_index += 1;
            } else if letter == 'l' && pattern_index == 2 {
                pattern_index += 1;
            } else if letter == '(' && pattern_index == 3 {
                pattern_index += 1;
                num_count = 0;
                first_num = "".to_string();
            } else if letter >= '0' && letter <= '9' && pattern_index == 4 {
                first_num.push(letter);
                num_count += 1;

                if num_count >= 4 {
                    // restart, invalid pattern
                    pattern_index = 0;
                }
            } else if letter == ',' && num_count > 0 && pattern_index == 4 {
                pattern_index += 1;
                num_count = 0;
                second_num = "".to_string();
            } else if letter >= '0' && letter <= '9' && pattern_index == 5 {
                second_num.push(letter);
                num_count += 1;

                if num_count >= 4 {
                    // restart, invalid pattern
                    pattern_index = 0;
                }
            } else if letter == ')' && num_count > 0 && pattern_index == 5 {
                answer += first_num.parse::<u64>().unwrap() * second_num.parse::<u64>().unwrap();
                pattern_index = 0;
            } else {
                pattern_index = 0;
            }
        }
    }

    return format!("{}", answer);
}


#[allow(non_snake_case)]
pub fn d03_part2(contents: String) -> String {
    let mut answer: u64 = 0;

    let mut pattern_index: u64 = 0;
    let mut num_count: u64 = 0;
    let mut first_num: String = "".to_string();
    let mut second_num: String = "".to_string();
    let mut enabled: bool = true;

    for (_line_num, line) in contents.lines().enumerate() {
        for (letter_pos, letter) in line.chars().enumerate() {
            // look for enabling and disabling sequence
            if line[letter_pos..].starts_with("do()") {
                enabled = true;
            } else if line[letter_pos..].starts_with("don't()") {
                enabled = false;
            }

            // only mul if enabled
            if enabled {
                if letter == 'm' && pattern_index == 0 {
                    pattern_index += 1;
                } else if letter == 'u' && pattern_index == 1 {
                    pattern_index += 1;
                } else if letter == 'l' && pattern_index == 2 {
                    pattern_index += 1;
                } else if letter == '(' && pattern_index == 3 {
                    pattern_index += 1;
                    num_count = 0;
                    first_num = "".to_string();
                } else if letter >= '0' && letter <= '9' && pattern_index == 4 {
                    first_num.push(letter);
                    num_count += 1;

                    if num_count >= 4 {
                        // restart, invalid pattern
                        pattern_index = 0;
                    }
                } else if letter == ',' && num_count > 0 && pattern_index == 4 {
                    pattern_index += 1;
                    num_count = 0;
                    second_num = "".to_string();
                } else if letter >= '0' && letter <= '9' && pattern_index == 5 {
                    second_num.push(letter);
                    num_count += 1;

                    if num_count >= 4 {
                        // restart, invalid pattern
                        pattern_index = 0;
                    }
                } else if letter == ')' && num_count > 0 && pattern_index == 5 {
                    answer += first_num.parse::<u64>().unwrap() * second_num.parse::<u64>().unwrap();
                    pattern_index = 0;
                } else {
                    pattern_index = 0;
                }
            } else {
                pattern_index = 0;
            }
        }
    }

    return format!("{}", answer);
}

#[allow(non_snake_case)]
pub fn d04_part1(contents: String) -> String {
    let mut answer: u64 = 0;
    let mut word_puzzle: Vec<String> = vec![];
    let search_directions: Vec<(isize, isize)> = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]; // top left to bottom right


    for (_line_num, line) in contents.lines().enumerate() {
        word_puzzle.push(line.to_string());
    }

    for i in 0..word_puzzle.len() {
        for j in 0..word_puzzle.first().unwrap().len() {
            if word_puzzle[i].chars().nth(j).unwrap() == 'X' {
                // check all 8 directions for the next letter 'M'
                for (i_offset, j_offset) in search_directions.clone() {
                    let mut matching_chars: usize = 0;
                    let mut curr_i: isize = i as isize;
                    let mut curr_j: isize = j as isize;

                    for letter in "MAS".chars() {
                        curr_i += i_offset;
                        curr_j += j_offset;

                        if curr_i >= 0 && curr_i < word_puzzle.len() as isize && 
                                curr_j >= 0 && curr_j < word_puzzle.first().unwrap().len() as isize && 
                                word_puzzle[curr_i as usize].chars().nth(curr_j as  usize).unwrap() == letter {
                            matching_chars += 1;
                        } else {
                            break;
                        }
                    }
                    
                    if matching_chars == 3 {
                        // we found one solution
                        answer += 1;
                    }
                }
            }
        }
    }

    return format!("{}", answer);
}

#[allow(non_snake_case)]
pub fn d04_part2(contents: String) -> String {
    let mut answer: u64 = 0;
    let mut word_puzzle: Vec<String> = vec![];
    let x1_search_directions: Vec<(isize, isize)> = vec![(-1, -1), (1, 1)]; // top left, bottom right
    let x2_search_directions: Vec<(isize, isize)> = vec![(1, -1), (-1, 1)]; // top right, bottom left

    for (_line_num, line) in contents.lines().enumerate() {
        word_puzzle.push(line.to_string());
    }

    for i in 0..word_puzzle.len() {
        for j in 0..word_puzzle.first().unwrap().len() {
            if word_puzzle[i].chars().nth(j).unwrap() == 'A' {
                let mut remaining_letters: Vec<char> = vec!['M', 'S'];
                let mut matching_chars: usize = 0;
                for (i_offset, j_offset) in x1_search_directions.clone() {
                    let curr_i: isize = i as isize + i_offset;
                    let curr_j: isize = j as isize + j_offset;

                    if curr_i >= 0 && curr_i < word_puzzle.len() as isize && 
                            curr_j >= 0 && curr_j < word_puzzle.first().unwrap().len() as isize && 
                            remaining_letters.contains(&word_puzzle[curr_i as usize].chars().nth(curr_j as  usize).unwrap()) {
                        matching_chars += 1;
                        remaining_letters.retain(|value| *value != word_puzzle[curr_i as usize].chars().nth(curr_j as  usize).unwrap());
                    } else {
                        break;
                    }
                }

                remaining_letters = vec!['M', 'S'];
                for (i_offset, j_offset) in x2_search_directions.clone() {
                    let curr_i: isize = i as isize + i_offset;
                    let curr_j: isize = j as isize + j_offset;

                    if curr_i >= 0 && curr_i < word_puzzle.len() as isize && 
                            curr_j >= 0 && curr_j < word_puzzle.first().unwrap().len() as isize && 
                            remaining_letters.contains(&word_puzzle[curr_i as usize].chars().nth(curr_j as  usize).unwrap()) {
                        matching_chars += 1;
                        remaining_letters.retain(|value| *value != word_puzzle[curr_i as usize].chars().nth(curr_j as  usize).unwrap());
                    } else {
                        break;
                    }
                }

                if matching_chars == 4 {
                    // we found one solution
                    answer += 1;
                }
            }
        }
    }

    return format!("{}", answer);
}

#[allow(non_snake_case)]
pub fn d05_part1(contents: String) -> String {
    let mut answer: u64 = 0;
    let mut page_ordering_rules: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut in_rules_section: bool = true;

    for (_line_num, line) in contents.lines().enumerate() {
        if line == "" {
            // switch to new section
            in_rules_section = false;
        } else {
            if in_rules_section == true {
                // read the rules
                let nums: Vec<u64> = line.to_string()
                    .split('|')
                    .map(|x: &str| x.parse::<u64>().unwrap())
                    .collect();

                match page_ordering_rules.get_mut(&nums[0]) {
                    Some(v) => {
                        v.push(nums[1]);
                    }
                    None => {
                        page_ordering_rules.insert(nums[0], vec![nums[1]]);
                    }
                }
            } else {
                // apply the rules
                let mut pages: Vec<u64> = vec![];

                for page in line.split(',') {
                    pages.push(page.parse::<u64>().unwrap());
                }

                let mut correct_i: bool = true;
                for i in 0..(pages.len()) {
                    match page_ordering_rules.get(&pages[i]) {
                        Some(after_pages) => {
                            let mut correct_j: bool = true;

                            // check everything after i to ensure it is in page_ordering_rules
                            for j in (i+1)..pages.len() {
                                if !after_pages.contains(&pages[j]) {
                                    correct_j = false;
                                    break;
                                }
                            }

                            if correct_j == false {
                                correct_i = false;
                                break;
                            }

                            // check everything before i to ensure it is NOT in page_ordering_rules
                            for j in 0..i {
                                if after_pages.contains(&pages[j]) {
                                    correct_j = false;
                                    break;
                                }
                            }

                            if correct_j == false {
                                correct_i = false;
                                break;
                            }
                        }  
                        None => {
                            // if not in ordering book assume it's correct?
                        }
                    }
                }

                if correct_i == true {
                    // add the middle value
                    answer += pages[pages.len() >> 1];
                }
            }
        }
    }

    return format!("{}", answer);
}

#[allow(non_camel_case_types)]
struct d05_custom_comparator {
    page_ordering_rules: HashMap<u64, Vec<u64>>
}

impl d05_custom_comparator {
    fn page_compare(&self, a: u64, b: u64) -> Ordering {
        let a_after_pages: Option<&Vec<u64>> = self.page_ordering_rules.get(&a);
        let b_after_pages: Option<&Vec<u64>> = self.page_ordering_rules.get(&b);
        let ret: Ordering;

        if a_after_pages.is_none() && b_after_pages.is_none() {
            ret = Ordering::Equal; // don't care
        } else if a_after_pages.is_some() && b_after_pages.is_none() {
            ret = Ordering::Less; // don't swap
        } else if a_after_pages.is_none() && b_after_pages.is_some() {
            ret = Ordering::Greater; // swap
        } else {
            // need to check if it contains the other
            if a_after_pages.unwrap().contains(&b) && !b_after_pages.unwrap().contains(&a) {
                ret = Ordering::Less; // don't swap
            } else if !a_after_pages.unwrap().contains(&b) && b_after_pages.unwrap().contains(&b) {
                ret = Ordering::Greater; // swap
            } else if !a_after_pages.unwrap().contains(&b) && !b_after_pages.unwrap().contains(&a) {
                ret = Ordering::Equal; // don't care
            } else {
                // if each number is found in the other's after pages, then it's contradicting itself
                // A < B && A > B
                // theoretically it shouldn't make it here but jsut in case
                ret = Ordering::Equal;
            }
        };

        return ret;
    }
}

#[allow(non_snake_case)]
pub fn d05_part2(contents: String) -> String {
    let mut answer: u64 = 0;
    let mut in_rules_section: bool = true;
    let mut comparator: d05_custom_comparator = d05_custom_comparator { 
        page_ordering_rules: HashMap::new() 
    };

    for (_line_num, line) in contents.lines().enumerate() {
        if line == "" {
            // switch to new section
            in_rules_section = false;
        } else {
            if in_rules_section == true {
                // read the rules
                let nums: Vec<u64> = line.to_string()
                    .split('|')
                    .map(|x: &str| x.parse::<u64>().unwrap())
                    .collect();

                match comparator.page_ordering_rules.get_mut(&nums[0]) {
                    Some(v) => {
                        v.push(nums[1]);
                    }
                    None => {
                        comparator.page_ordering_rules.insert(nums[0], vec![nums[1]]);
                    }
                }
            } else {
                // apply the rules
                let mut pages: Vec<u64> = vec![];

                for page in line.split(',') {
                    pages.push(page.parse::<u64>().unwrap());
                }

                // sort the pages
                let original_pages: Vec<u64> = pages.clone();
                pages.sort_by(|a, b| comparator.page_compare(*a, *b));

                if original_pages != pages {
                    // only sum the middle values of the incorrectly sorted lists
                    answer += pages[pages.len() >> 1];
                }
            }
        }
    }

    return format!("{}", answer);
}

#[allow(non_snake_case)]
fn day06_turn_right(p: Position) -> Position {
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
pub fn d06_part1(contents: String) -> String {
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
                direction_offset = day06_turn_right(direction_offset);
            } else {
                // move forward
                lab_guard_position = new_lab_guard_position;
                
                if !visited_squares.contains(&lab_guard_position) {
                    visited_squares.insert(lab_guard_position);
                    answer += 1;
                }
            }
    }

    return format!("{}", answer);
}


#[allow(non_snake_case)]
pub fn d06_part2(contents: String) -> String {
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
                        direction_offset = day06_turn_right(direction_offset);
                    } else {
                        // move forward
                        lab_guard_position = new_lab_guard_position;
                    }

                    if loop_count >= 8000 {
                        // in a loop
                        answer += 1;
                        break;
                    }

                    loop_count = loop_count.saturating_add(1);
                }
            }
        }
    }

    return format!("{}", answer);
}


#[allow(non_snake_case)]
pub fn d07_part1(contents: String) -> String {
    let mut answer: u64 = 0;

    for (_line_num, line) in contents.lines().enumerate() {
        let total: u64 = line.split(':')
            .nth(0).unwrap()
            .parse::<u64>().unwrap();
        let coefficients: Vec<u64> = line.to_string()
            .split(':')
            .nth(1).unwrap()
            .split_ascii_whitespace()
            .map(|x: &str| x.parse::<u64>().unwrap())
            .collect();


        let mut loop_count: u64 = 0;
        while loop_count < (1 << coefficients.len()) {
            let mut current_evaluation: u64 = coefficients[0];
            for i in 1..coefficients.len() {
                // check each bit in loop count to see if we should add or multiply
                if ((1 << i) & loop_count) == 0 {
                    // 0b0 is an add
                    current_evaluation += coefficients[i];
                } else {
                    // 0b1 is a multiply
                    current_evaluation *= coefficients[i];
                }
            }

            if current_evaluation == total {
                answer += total;
                break;
            }

            loop_count = loop_count.saturating_add(1);
        }
    }

    return format!("{}", answer);
}

fn day07_concat(a: u64, b: u64) -> u64 { 
    return a as u64 * 10u64.pow(b.ilog10() + 1) + b as u64;
}

#[allow(non_snake_case)]
pub fn d07_part2(contents: String) -> String {
    let mut answer: u64 = 0;

    for (_line_num, line) in contents.lines().enumerate() {
        let total: u64 = line.split(':')
            .nth(0).unwrap()
            .parse::<u64>().unwrap();
        let coefficients: Vec<u64> = line.to_string()
            .split(':')
            .nth(1).unwrap()
            .split_ascii_whitespace()
            .map(|x: &str| x.parse::<u64>().unwrap())
            .collect();

        // (number of values to select).map(|| range of values to select from).generate permutations with repition
        for perm in (0..coefficients.len()-1).map(|_| 0..3).multi_cartesian_product() {
            let mut current_evaluation: u64 = coefficients[0];
            for (n, p) in perm.iter().enumerate() {
                // check each bit in loop count to see if we should add or multiply
                if *p == 0 {
                    // remainder of 0 is add
                    current_evaluation += coefficients[n+1];
                } else if *p == 1 {
                    // remainder of 1 is multiply
                    current_evaluation *= coefficients[n+1];
                } else {
                    // remainder of 2 is concatenate
                    current_evaluation = day07_concat(current_evaluation, coefficients[n+1]);
                }

                if current_evaluation > total {
                    break;
                }
            }

            if current_evaluation == total {
                answer += total;
                break;
            }
        }
    }

    return format!("{}", answer);
}

#[allow(non_snake_case)]
pub fn d08_part1(contents: String) -> String {
    let mut answer: u64 = 0;
    let mut antennas: HashMap<char, Vec<Position>> = HashMap::new();
    let mut raw_antennas: HashSet<Position> = HashSet::new();
    let mut map_width: u64 = 0;
    let mut map_height: u64 = 0;
    let mut antinodes: HashMap<char, Vec<Position>> = HashMap::new();

    for (row_num, line) in contents.lines().enumerate() {
        for (col_num, c) in line.chars().enumerate() {
            if c != '.' {
                // found antenna, add to the map
                let new_antenna: Position = Position::new( col_num as i32, row_num as i32);
                match antennas.get_mut(&c) {
                    Some(v) => {
                        v.push(new_antenna);
                    }
                    None => {
                        antennas.insert(c, vec![new_antenna]);
                    }
                }

                if !raw_antennas.contains(&new_antenna) {
                    raw_antennas.insert(new_antenna);
                }
            }

            if col_num as u64 > map_width {
                map_width = col_num as u64;
            }
        }

        if row_num as u64 > map_height {
            map_height = row_num as u64;
        }
    }

    // loop through each antenna type
    for (frequency, antenna_locs) in antennas.iter() {
        // (number of values to select).map(|| range of values to select from).generate permutations with repition
        for perm in (0..antenna_locs.len()).permutations(2) {
            let dx = antenna_locs[perm[0]].x - antenna_locs[perm[1]].x;
            let dy = antenna_locs[perm[0]].y - antenna_locs[perm[1]].y;

            let new_antinodes: [Position; 2] = [
                Position::new(antenna_locs[perm[0]].x + dx, antenna_locs[perm[0]].y + dy),
                Position::new(antenna_locs[perm[1]].x - dx, antenna_locs[perm[1]].y - dy)
            ];

            for antinode in new_antinodes {
                if ((antinodes.contains_key(&frequency) && !antinodes.get(frequency).unwrap().contains(&antinode)) || !antinodes.contains_key(&frequency)) && 
                        antinode.x <= map_width as i32 && antinode.y <= map_height as i32 && 
                        antinode.x >= 0 && antinode.y >= 0 {
                    // not already an antinode fo rthat frequency, not an antenna, and inside the map
                    // add to antinodes
                    match antinodes.get_mut(&frequency) {
                        Some(v) => {
                            v.push(antinode);
                        }
                        None => {
                            antinodes.insert(*frequency, vec![antinode]);
                        }
                    }
                }
            }
        }
    }

    // draw antinodes
    for i in 0..=map_height {
        for j in 0..=map_width {
            for k in antinodes.keys() {
                if antinodes[k].contains(&Position::new(j as i32, i as i32)) {
                    answer += 1;
                    break;
                }
            }
        }
    }

    return format!("{}", answer);
}


#[allow(non_snake_case)]
pub fn d08_part2(contents: String) -> String {
    let mut answer: u64 = 0;
    let mut antennas: HashMap<char, Vec<Position>> = HashMap::new();
    let mut raw_antennas: HashSet<Position> = HashSet::new();
    let mut map_width: u64 = 0;
    let mut map_height: u64 = 0;
    let mut antinodes: HashMap<char, Vec<Position>> = HashMap::new();

    for (row_num, line) in contents.lines().enumerate() {
        for (col_num, c) in line.chars().enumerate() {
            if c != '.' {
                // found antenna, add to the map
                let new_antenna: Position = Position::new( col_num as i32, row_num as i32);
                match antennas.get_mut(&c) {
                    Some(v) => {
                        v.push(new_antenna);
                    }
                    None => {
                        antennas.insert(c, vec![new_antenna]);
                    }
                }

                if !raw_antennas.contains(&new_antenna) {
                    raw_antennas.insert(new_antenna);
                }
            }

            if col_num as u64 > map_width {
                map_width = col_num as u64;
            }
        }

        if row_num as u64 > map_height {
            map_height = row_num as u64;
        }
    }

    // loop through each antenna type
    for (frequency, antenna_locs) in antennas.iter() {
        // (number of values to select).map(|| range of values to select from).generate permutations with repition
        for perm in (0..antenna_locs.len()).permutations(2) {
            let dx = antenna_locs[perm[0]].x - antenna_locs[perm[1]].x;
            let dy = antenna_locs[perm[0]].y - antenna_locs[perm[1]].y;

            for direction in [-1, 1] {
                let mut antinode: Position = Position::new(antenna_locs[perm[0]].x + (direction*dx), antenna_locs[perm[0]].y + (direction*dy));

                while antinode.x >= 0 && antinode.y >= 0 && antinode.x <= map_width as i32 && antinode.y <= map_height as i32 {
                    if ((antinodes.contains_key(&frequency) && !antinodes.get(frequency).unwrap().contains(&antinode)) || !antinodes.contains_key(&frequency)) && 
                            antinode.x <= map_width as i32 && antinode.y <= map_height as i32 && 
                            antinode.x >= 0 && antinode.y >= 0 {
                        // not already an antinode fo rthat frequency, not an antenna, and inside the map
                        // add to antinodes
                        match antinodes.get_mut(&frequency) {
                            Some(v) => {
                                v.push(antinode);
                            }
                            None => {
                                antinodes.insert(*frequency, vec![antinode]);
                            }
                        }
                    }

                    // update antinode
                    antinode.x += direction*dx;
                    antinode.y += direction*dy;
                }
            }
        }
    }

    // draw antinodes
    for i in 0..=map_height {
        for j in 0..=map_width {
            for k in antinodes.keys() {
                if antinodes[k].contains(&Position::new(j as i32, i as i32)) {
                    answer += 1;
                    break;
                }
            }
        }
    }

    return format!("{}", answer);
}

#[allow(non_snake_case)]
pub fn d09_part1(contents: String) -> String {
    let mut answer: u64 = 0;

    for (_line_num, line) in contents.lines().enumerate() {

        // filesystem[value] = (starting_index, repeats)
        let mut filesystem: Vec<(u64, u8)> = Vec::new();
        let mut start_index: u64 = 0;

        for (value, repeats) in line.chars().enumerate() {
            let reps: u8 = repeats.to_digit(10).unwrap() as u8;

            if (value & 0x01) == 0x00 {
                // only save even value indices (odds are assumed to be empty space)
                filesystem.push((start_index, reps));
            }

            // get the next starting index
            start_index = start_index + reps as u64;
        }

        let mut head_index: u64 = 0;
        let mut tail_index: u64 = (filesystem.len()-1) as u64;
        let mut compact_filesystem: Vec<u64> = Vec::new();
        while head_index < tail_index {
            let (start_index, repetitions) = filesystem[head_index as usize];
            let empty_space_start: u64 = start_index + repetitions  as u64;
            let empty_space_end: u64 = filesystem[(head_index + 1) as usize].0;

            // add the unmoved bytes
            for _ in start_index..start_index+repetitions as u64 {
                compact_filesystem.push(head_index);
            }
            
            // add the moved bytes into the compact file system
            let mut i: u64 = 0;
            while i < (empty_space_end - empty_space_start) {
                if filesystem[tail_index as usize].1 == 0 {
                    // no more repetitions
                    // remove the last index from filesystem and decrement tail_index
                    // and add the next value
                    _ = filesystem.pop();
                    tail_index -= 1;
                }

                compact_filesystem.push(tail_index);

                if filesystem[tail_index as usize].1 > 0 {
                    // decrement the repetitions of the value
                    filesystem[tail_index as usize].1 -= 1;
                    i += 1;
                }

                if filesystem[tail_index as usize].1 == 0 {
                    // no more repetitions
                    // remove the last index from filesystem and decrement tail_index
                    // and add the next value
                    _ = filesystem.pop();
                    tail_index -= 1;
                }

                if head_index >= tail_index {
                    break;
                }
            }

            head_index = head_index.saturating_add(1);
        }

        // append any remaining repetitions from head_index to compact_filesystem
        if head_index as usize <= filesystem.len() - 1 && 
                filesystem[head_index as usize].1 > 0 {
            for _ in 0..filesystem[head_index as usize].1 {
                compact_filesystem.push(head_index);
            }
        }

        for (i, val) in compact_filesystem.into_iter().enumerate() {
            answer += (i as u64)*val;
        }

        break;
    }

    return format!("{}", answer);
}


#[allow(non_snake_case)]
pub fn d09_part2(contents: String) -> String {
    let mut answer: u64 = 0;

    for (_line_num, line) in contents.lines().enumerate() {

        // filesystem[idx] = (starting_index, repeats, value)
        let mut filesystem: Vec<(u64, u8, u64)> = Vec::new();
        let mut start_index: u64 = 0;
        let mut file_contents: u64 = 0;
        for (value, repeats) in line.chars().enumerate() {
            let reps: u8 = repeats.to_digit(10).unwrap() as u8;

            if (value & 0x01) == 0x00 {
                // only save even value indices (odds are assumed to be empty space)
                filesystem.push((start_index, reps, file_contents));
                file_contents += 1;
            }

            // get the next starting index
            start_index = start_index + reps as u64;
        }

        let mut tail_index: u64 = (filesystem.len()-1) as u64;
        while tail_index > 0 {
            let mut early_exit: bool = false;
            let mut head_index: u64 = 0;
            while head_index < tail_index {
                let (start_index, repetitions, _) = filesystem[head_index as usize];
                let empty_space_start: u64 = start_index + repetitions  as u64;
                let empty_space_end: u64 = filesystem[(head_index + 1) as usize].0;
                let remaining_size: u64 = empty_space_end - empty_space_start;

                // try to fit the tail into the front most empty spot
                if filesystem[tail_index as usize].1 as u64 <= remaining_size {
                    // pop the last value off as it's no longer at the end
                    let mut back_spot: (u64, u8, u64) = filesystem.remove(tail_index as usize);

                    // update the new starting location
                    back_spot.0 = empty_space_start;

                    // move it to this spot and then stop trying to find a new spot for the file
                    filesystem.insert((head_index + 1) as usize, back_spot);
                    early_exit = true;
                    break;
                }

                // try the next empty spot
                head_index += 1;
            }

            if early_exit == false {
                // there is no room for the tail file, leave the file where it is
                tail_index -= 1;
            }
        }


        for (starting_index, length, file_contents) in filesystem.into_iter() {
            for j in 0..length {
                answer += (starting_index + j as u64)*(file_contents as u64);
            }
        }

        break;
    }

    return format!("{}", answer);
}

#[allow(non_snake_case)]
fn day10_optimized_dijkstras_search(  weighted_map: &Vec<Vec<u8>>, start: Position, 
                                    goal: Position ) -> Vec<Position> {
    let mapWidth: usize = weighted_map[0].len();
    let mapHeight: usize = weighted_map.len();

    let mut path: Vec<Position> = Vec::with_capacity(1 as usize);
    if start.x < 0 || start.y < 0 || goal.x >= mapWidth as i32 || goal.y >= mapHeight as i32 ||
       start == goal || mapWidth < 2 || mapHeight < 2 {
        return path;
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
                    weighted_map[neighbor.y as usize][neighbor.x as usize] == weighted_map[current.y as usize][current.x as usize].saturating_add(1) {
                let neighbor_gscore: f32 = *gscore.get(&current).unwrap_or(&0.0) + weighted_map[neighbor.y as usize][neighbor.x as usize] as f32 + 
                                            day10_optimized_heuristic(neighbor, current);

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
        match came_from.get(&current) {
            Some(x) => {
                current = *x;
            } None => {
                break;
            }
        }
    }

    path.reverse();


    return path;
}


#[inline]
fn day10_optimized_heuristic(a: Position, b: Position) -> f32 {
    return (((a.x - b.x) + (a.y - b.y)) as f32).abs();
}

#[allow(non_snake_case)]
pub fn d10_part1(contents: String) -> String {
    let mut answer: u64 = 0;
    let mut maze: Vec<Vec<u8>> = Vec::new();
    let mut start_positions: Vec<Position> = Vec::new();
    let mut end_positions: Vec<Position> = Vec::new();

    for (row_num, line) in contents.lines().enumerate() {
        let mut row: Vec<u8> = Vec::new();
        for (col_num, c) in line.chars().enumerate() {
            if c == '0' {
                start_positions.push(Position::new(col_num as i32, row_num as i32));
            } else if c == '9' {
                end_positions.push(Position::new(col_num as i32, row_num as i32));
            }

            row.push(c.to_digit(10).unwrap() as u8);
        }
        maze.push(row);
    }

    for start in start_positions {
        for end in end_positions.clone() {
            let path: Vec<Position> = day10_optimized_dijkstras_search(&maze, start, end);

            if path.len() == 9 {
                answer += 1;
            }
        }
    }

    return format!("{}", answer);
}

#[allow(non_snake_case)]
fn day10_depth_first_search(weighted_map: &Vec<Vec<u8>>, start: Position, count: &mut u64) {
    let mapWidth: usize = weighted_map[0].len();
    let mapHeight: usize = weighted_map.len();

    if start.x < 0 || start.y < 0 || mapWidth < 2 || mapHeight < 2 {
        return;
    }

    for neighbor in start.get_surrounding_positions() {
        // if the neighbor is a valid position
        if neighbor.x >= 0 && neighbor.y >= 0 && 
                neighbor.y < mapHeight as i32 && neighbor.x < mapWidth as i32 &&
                weighted_map[neighbor.y as usize][neighbor.x as usize] == weighted_map[start.y as usize][start.x as usize].saturating_add(1) {
            
            if weighted_map[neighbor.y as usize][neighbor.x as usize] == 9 {
                *count += 1;
            } else {
                day10_depth_first_search(weighted_map, neighbor, count);
            }
        }
    }

    return;
}

#[allow(non_snake_case)]
pub fn d10_part2(contents: String) -> String {
    let mut answer: u64 = 0;
    let mut maze: Vec<Vec<u8>> = Vec::new();
    let mut start_positions: Vec<Position> = Vec::new();

    for (row_num, line) in contents.lines().enumerate() {
        let mut row: Vec<u8> = Vec::new();
        for (col_num, c) in line.chars().enumerate() {
            if c == '0' {
                start_positions.push(Position::new(col_num as i32, row_num as i32));
            }

            row.push(c.to_digit(10).unwrap() as u8);
        }
        maze.push(row);
    }

    for start in start_positions {
        day10_depth_first_search(&maze, start, &mut answer);
    }

    return format!("{}", answer);
}

#[allow(non_snake_case)]
pub fn d11_part1(contents: String) -> String {
    let mut arrangement: Vec<u64> = Vec::new();

    for (_line_num, line) in contents.lines().enumerate() {
        arrangement = line.split_ascii_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
    }

    for _ in 0..25 {
        let mut new_arrangement: Vec<u64> = Vec::new();
        for j in 0..arrangement.len() {
            if arrangement[j] == 0 {
                new_arrangement.push(1);
            } else if arrangement[j].to_string().len() & 0x01 == 0x00 {
                // even number of digits
                let num_as_str: String = arrangement[j].to_string();
                let num_digits: usize = num_as_str.len() >> 1;
                let left: u64 = num_as_str[..num_digits].parse::<u64>().unwrap();
                let right: u64 = num_as_str[num_digits..].parse::<u64>().unwrap();
                new_arrangement.push(left);
                new_arrangement.push(right);
            } else {
                new_arrangement.push(arrangement[j] * 2024);
            }
        }

        arrangement = new_arrangement;
    }

    return format!("{}", arrangement.len());
}


fn day11_get_num_digits(mut num: u64) -> u64 {
    let mut num_digits: u64 = 1;
    while (num / 10) > 0 {
        num_digits += 1;
        num /= 10;
    }
    return num_digits;
}

fn day11_split_digits(mut num: u64, num_digits: u64) -> (u64, u64) {
    let mut left: u64 = 0;
    let mut right: u64 = 0;
    let half_point: u64  = num_digits >> 1;

    let mut right_count: u64 = 0;
    let mut left_count: u64 = 0;
    for i in 0..num_digits {
        if i < half_point {
            right += (num % 10) * (10 as u64).pow(right_count as u32);
            right_count += 1;
        } else {
            left += (num % 10) * (10 as u64).pow(left_count as u32);
            left_count += 1;
        }
        num /= 10;
    }

    return (left, right);
}

#[allow(non_snake_case)]
pub fn d11_part2(contents: String) -> String {
    let mut answer: u64 = 0;
    let mut arrangement: HashMap<u64, u64> = HashMap::new();

    for (_line_num, line) in contents.lines().enumerate() {
        for num in line.split_ascii_whitespace() {
            let digit: u64 = num.parse::<u64>().unwrap();

            match arrangement.get_mut(&digit) {
                Some(x) => {
                    *x += 1;
                } None => {
                    arrangement.insert(digit, 1);
                }
            }
        }
    }

    for _ in 0..75 {
        let mut new_arrangement: HashMap<u64, u64> = HashMap::new();
        for (digit, count) in arrangement.iter() {
            if *digit == 0 {
                // CONVERT to a 1
                let new_digit: u64 = 1;
                match new_arrangement.get_mut(&new_digit) {
                    Some(x) => {
                        // increment the current count, if the digit is already in the hashmap
                        *x += *count;
                    } None => {
                        // if not in the hashmap, add it with the current count
                        new_arrangement.insert(new_digit, *count);
                    }
                }
            } else if day11_get_num_digits(*digit) & 0x01 == 0x00 {
                // even number of digits
                let num_digits: u64 = day11_get_num_digits(*digit);
                let (left, right): (u64, u64) = day11_split_digits(*digit, num_digits);

                for d in [left, right] {
                    match new_arrangement.get_mut(&d) {
                        Some(x) => {
                            // increment the current count, if the digit is already in the hashmap
                            *x += *count;
                        } None => {
                            // if not in the hashmap, add it with the current count
                            new_arrangement.insert(d, *count);
                        }
                    }
                }
            } else {
                // mutliply by 2024
                let new_digit: u64 = digit * 2024;
                match new_arrangement.get_mut(&new_digit) {
                    Some(x) => {
                        // increment the current count, if the digit is already in the hashmap
                        *x += *count;
                    } None => {
                        // if not in the hashmap, add it with the current count
                        new_arrangement.insert(new_digit, *count);
                    }
                }
            }
        }

        // overwrite the old arrangement with the newly constructed one
        arrangement = new_arrangement;
    }

    for (_digit, count) in arrangement.iter() {
        answer += count;
    }

    return format!("{}", answer);
}

#[allow(non_camel_case_types)]
struct day12_GardenPlot {
    perimeter: u64,
    locations: HashSet<Position, PositionBuildHasher>,
}

impl day12_GardenPlot {
    fn new(p: u64, l: HashSet<Position, PositionBuildHasher>) -> Self {
        Self {perimeter: p, locations: l}
    }
}

#[allow(non_snake_case)]
fn day12_BreadthFirstSearch(map: &Vec<Vec<char>>, start: Position, garden: &mut HashMap<char, Vec<day12_GardenPlot>>, plant: char) {
    let map_height: u64 = map.len() as u64;
    let map_width: u64 = map.first().unwrap().len() as u64;

    let mut open: VecDeque<Position> = VecDeque::new();
    let mut closed: HashSet<Position, PositionBuildHasher> = HashSet::with_hasher(PositionBuildHasher);

    // add a new plot
    match garden.get_mut(&plant) {
        Some(x) => {
            // push a new garden plot
            let new_locations: HashSet<Position, PositionBuildHasher> = HashSet::with_hasher(PositionBuildHasher);
            let new_plot: day12_GardenPlot = day12_GardenPlot::new(0, new_locations);
            x.push(new_plot);
        } None => {
            // insert the first garden plot
            let new_locations: HashSet<Position, PositionBuildHasher> = HashSet::with_hasher(PositionBuildHasher);
            let new_plot: day12_GardenPlot = day12_GardenPlot::new(0, new_locations);
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
                x.last_mut().unwrap().locations.insert(current);
            } None => {
                // should never be reached, but insert just in case
                let mut locations: HashSet<Position, PositionBuildHasher> = HashSet::with_hasher(PositionBuildHasher);
                locations.insert(current);
                garden.insert(plant, vec![day12_GardenPlot::new(0, locations)]);
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
                            garden.insert(plant, vec![day12_GardenPlot::new(1, locations)]);
                        }
                    }
                }
            }
        }
    }
}


#[allow(non_snake_case)]
pub fn d12_part1(contents: String) -> String {
    let mut garden: HashMap<char, Vec<day12_GardenPlot>> = HashMap::new();
    let mut map: Vec<Vec<char>> = Vec::new();

    // build map
    for (_col_num, line) in contents.lines().enumerate() {
        let mut garden_row: Vec<char> = Vec::new();
        for (_row_num, c) in line.chars().enumerate() {
            garden_row.push(c);

            if !garden.contains_key(&c) {
                let l: HashSet<Position, PositionBuildHasher> = HashSet::with_hasher(PositionBuildHasher);
                garden.insert(c, vec![day12_GardenPlot::new(0, l)]);
            }
        }

        map.push(garden_row);
    }

    // loop through the map and find all plots
    for (row_num, row) in map.iter().enumerate() {
        for (col_num, plant) in row.iter().enumerate() {
            let plant_loc: Position = Position::new(col_num as i32, row_num as i32);
            let plots: Option<&Vec<day12_GardenPlot>> = garden.get(plant);

            // this plant species has not been discovered or this specific location has not been visited yet
            if plots.is_none() {
                day12_BreadthFirstSearch(&map, plant_loc, &mut garden, *plant);
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
                    day12_BreadthFirstSearch(&map, plant_loc, &mut garden, *plant);
                }
            }
        }
    }

    let mut answer: u64 = 0;
    for (_, plots) in garden.iter() {
        for plot in plots {
            answer += plot.locations.len() as u64 * plot.perimeter;
        }
    }
    return format!("{}", answer);
}


#[allow(non_snake_case)]
pub fn d12_part2(contents: String) -> String {
    let mut garden: HashMap<char, Vec<day12_GardenPlot>> = HashMap::new();
    let mut map: Vec<Vec<char>> = Vec::new();

    // build map
    for (_col_num, line) in contents.lines().enumerate() {
        let mut garden_row: Vec<char> = Vec::new();
        for (_row_num, c) in line.chars().enumerate() {
            garden_row.push(c);

            if !garden.contains_key(&c) {
                let l: HashSet<Position, PositionBuildHasher> = HashSet::with_hasher(PositionBuildHasher);
                garden.insert(c, vec![day12_GardenPlot::new(0, l)]);
            }
        }

        map.push(garden_row);
    }

    // loop through the map and find all plots
    for (row_num, row) in map.iter().enumerate() {
        for (col_num, plant) in row.iter().enumerate() {
            let plant_loc: Position = Position::new(col_num as i32, row_num as i32);
            let plots: Option<&Vec<day12_GardenPlot>> = garden.get(plant);

            // this plant species has not been discovered or this specific location has not been visited yet
            if plots.is_none() {
                day12_BreadthFirstSearch(&map, plant_loc, &mut garden, *plant);
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
                    day12_BreadthFirstSearch(&map, plant_loc, &mut garden, *plant);
                }
            }
        }
    }

    let mut answer: u64 = 0;
    for (_, plots) in garden.iter() {
        for plot in plots {
            let mut sides: u64 = 0;
            for p in plot.locations.iter() {
                for (p1, p2) in [
                            (Position::new(0, -1), Position::new(1, 0)), // north and east
                            (Position::new(0,  1), Position::new(1, 0)), // south and east
                            (Position::new(0,  1), Position::new(-1, 0)), // south and west
                            (Position::new(0,  -1), Position::new(-1, 0)),] { // north and west
                    let p1_mod: Position = p1 + *p;
                    let p2_mod: Position = p2 + *p;
                    let diag_mod: Position = p1 + p2 + *p;

                    let mut same_count: u8 = 0;
                    let mut different_count: u8 = 0;
                    for p_mod in [p1_mod, p2_mod] {
                        if p_mod.x < 0 || p_mod.y < 0 || p_mod.x >= map.first().unwrap().len() as i32 || p_mod.y >= map.len() as i32 ||
                                map[p_mod.y as usize][p_mod.x as usize] != map[p.y as usize][p.x as usize] {
                            // exterior corner
                            different_count += 1;
                        } else if p_mod.x >= 0 && p_mod.y >= 0 && p_mod.x < map.first().unwrap().len() as i32 && p_mod.y < map.len() as i32 &&
                                diag_mod.x >= 0 && diag_mod.y >= 0 && diag_mod.x < map.first().unwrap().len() as i32 && diag_mod.y < map.len() as i32 &&
                                map[p_mod.y as usize][p_mod.x as usize] == map[p.y as usize][p.x as usize] && 
                                map[diag_mod.y as usize][diag_mod.x as usize] != map[p.y as usize][p.x as usize] {
                            // interior corner
                            same_count += 1;
                        }
                    }

                    if same_count == 2 || different_count == 2 {
                        sides += 1;
                        //break;
                    }
                }
            }

            answer += sides * plot.locations.len() as u64;
        }
    }
    return format!("{}", answer);
}


#[allow(non_snake_case)]
pub fn d13_part1(contents: String) -> String {
    const BUTTON_A_COST: f64 = 3.0;
    const BUTTON_B_COST: f64 = 1.0;

    let mut answer: u64 = 0;
    let mut a_x: u64 = 0;
    let mut a_y: u64 = 0;

    let mut b_x: u64 = 0;
    let mut b_y: u64 = 0;

    let mut prize_x: u64 = 0;
    let mut prize_y: u64 = 0;

    for (_line_num, line) in contents.lines().enumerate() {
        if line.starts_with("Button A: ") {
            let raw_equation: &str = line.strip_prefix("Button A: X+").unwrap();
            a_x = raw_equation.split(',').nth(0).unwrap()
                                .parse::<u64>().unwrap();
            a_y = raw_equation.split(',').nth(1).unwrap()
                                .strip_prefix(" Y+").unwrap()
                                .parse::<u64>().unwrap();
        } else if line.starts_with("Button B: ") {
            let raw_equation: &str = line.strip_prefix("Button B: X+").unwrap();
            b_x = raw_equation.split(',').nth(0).unwrap()
                                .parse::<u64>().unwrap();
            b_y = raw_equation.split(',').nth(1).unwrap()
                                .strip_prefix(" Y+").unwrap()
                                .parse::<u64>().unwrap();
        } else if line.starts_with("Prize:") {
            let raw_equation: &str = line.strip_prefix("Prize: X=").unwrap();
            prize_x = raw_equation.split(',').nth(0).unwrap()
                                    .parse::<u64>().unwrap();
            prize_y = raw_equation.split(',').nth(1).unwrap()
                                    .strip_prefix(" Y=").unwrap()
                                    .parse::<u64>().unwrap();
        } else {
            // see how many buttons presses it takes to reach the target
            // determine which is more efficient to optimize (a or b) by maximizing the button that has
            // the biggest distance/ticket 
            let a_distance: f64 = ((a_x*a_x + a_y*a_y) as f64).sqrt();
            let b_distance: f64 = ((b_x*b_x + b_y*b_y) as f64).sqrt();
            let maximize_b: bool = a_distance / BUTTON_A_COST <= b_distance / BUTTON_B_COST;
            
            let numaxpresses: f64 = prize_x as f64 / a_x as f64;
            let numbxpresses: f64 = prize_x as f64 / b_x as f64;
            let numaypresses: f64 = prize_y as f64 / a_y as f64;
            let numbypresses: f64 = prize_y as f64 / b_y as f64;

            // maximize either a or b in the below equations 
            // depending on which give more distance / cost
            // xposition = a_x * a + b_x * b
            // yposition = a_y * a + b_y * b
            // cost = a*3 + b*1
            if maximize_b {
                let maxbpresses: u64 = numbxpresses.min(numbypresses).floor() as u64;

                let mut bpresses: u64 = maxbpresses;
                loop {
                    // solving for a in -> xposition = a_x * a + b_x * b
                    let apresses: u64 = (prize_x.saturating_sub(bpresses * b_x)) / a_x as u64; 

                    // now plug and chug to see if we actually hit the target
                    if a_x*apresses + b_x*bpresses == prize_x && a_y*apresses + b_y*bpresses == prize_y {
                        // found a winner, calculate the cost
                        let cost: u64 = apresses*(BUTTON_A_COST as u64) + bpresses*(BUTTON_B_COST as u64);
                        answer += cost;
                        break;
                    }

                    if bpresses == 0 {
                        break;
                    } else {
                        bpresses -= 1;
                    }
                }
            } else {
                let maxapresses: u64 = numaxpresses.min(numaypresses).floor() as u64;

                let mut apresses: u64 = maxapresses;
                loop {
                    // solve for b in -> xposition = a_x * a + b_x * b
                    let bpresses: u64 = (prize_x.saturating_sub(apresses * a_x)) / b_x as u64;

                    // now plug and chug to see if we actually hit the target
                    if a_x*apresses + b_x*bpresses == prize_x && a_y*apresses + b_y*bpresses == prize_y {
                        // found a winner, calcualte the cost
                        let cost: u64 = apresses*(BUTTON_A_COST as u64) + bpresses*(BUTTON_B_COST as u64);
                        answer += cost;
                    }

                    if apresses == 0 {
                        break;
                    } else {
                        apresses -= 1;
                    }
                }
            }

            // reset the variables
            a_x = 0;
            a_y = 0;
            b_x = 0;
            b_y = 0;
            prize_x = 0;
            prize_y = 0;
        }
    }

    return format!("{}", answer);
}



#[allow(non_snake_case)]
pub fn d13_part2(contents: String) -> String {
    const BUTTON_A_COST: f64 = 3.0;
    const BUTTON_B_COST: f64 = 1.0;

    let mut answer: u64 = 0;
    let mut a_x: u64 = 0;
    let mut a_y: u64 = 0;

    let mut b_x: u64 = 0;
    let mut b_y: u64 = 0;

    for (_line_num, line) in contents.lines().enumerate() {
        if line.starts_with("Button A: ") {
            let raw_equation: &str = line.strip_prefix("Button A: X+").unwrap();
            a_x = raw_equation.split(',').nth(0).unwrap()
                                .parse::<u64>().unwrap();
            a_y = raw_equation.split(',').nth(1).unwrap()
                                .strip_prefix(" Y+").unwrap()
                                .parse::<u64>().unwrap();
        } else if line.starts_with("Button B: ") {
            let raw_equation: &str = line.strip_prefix("Button B: X+").unwrap();
            b_x = raw_equation.split(',').nth(0).unwrap()
                                .parse::<u64>().unwrap();
            b_y = raw_equation.split(',').nth(1).unwrap()
                                .strip_prefix(" Y+").unwrap()
                                .parse::<u64>().unwrap();
        } else if line.starts_with("Prize:") {
            let raw_equation: &str = line.strip_prefix("Prize: X=").unwrap();
            let prize_x = raw_equation.split(',').nth(0).unwrap()
                                    .parse::<u64>().unwrap() + 10_000_000_000_000;
            let prize_y = raw_equation.split(',').nth(1).unwrap()
                                    .strip_prefix(" Y=").unwrap()
                                    .parse::<u64>().unwrap() + 10_000_000_000_000;

            // a_x*apresses + b_x*bpresses == prize_x
            // a_x*apresses = prize_x - (b_x * bpresses)
            // apresses = (prize_x - (b_x * bpresses)) / a_x

            // a_y*apresses + b_y*bpresses == prize_y
            // a_y*((prize_x - (b_x * bpresses)) / a_x) + b_y*bpresses = prize_y
            // a_y*prize_x/a_x - a_y*b_x*bpresses/a_x + b_y*bpresses = prize_y
            // bpresses*(-1*a_y*b_x/a_x + b_y) = prize_y - a_y*prize_x/a_x
            // bpresses = (prize_y - a_y*prize_x/a_x) / (-1*a_y*b_x/a_x + b_y)
            // bpresses = (prize_y - a_y*prize_x/a_x) / (b_y - a_y*b_x/a_x)

            let bpresses: u64 = ((prize_y as f64 - a_y as f64*prize_x as f64/a_x as f64) / 
                                (b_y as f64 - a_y as f64*b_x as f64/a_x as f64)).round() as u64;
            let apresses: u64 = ((prize_x as f64 - (b_x as f64 * bpresses as f64)) / a_x as f64).round() as u64;

            // plug and chug to see if we actually hit the target
            if //apresses >= 0 && bpresses >= 0 && 
                    a_x*apresses as u64 + b_x*bpresses as u64 == prize_x && 
                    a_y*apresses as u64 + b_y*bpresses as u64 == prize_y {
                // found a winner, calculate the cost
                let cost: u64 = (apresses as u64)*(BUTTON_A_COST as u64) + (bpresses as u64)*(BUTTON_B_COST as u64);
                answer += cost;
            }
        }
    }

    return format!("{}", answer);
}


#[allow(non_snake_case)]
pub fn d14_part1(contents: String) -> String {
    let width: u64 = 101;
    let height: u64 = 103;
    let seconds: u64 = 100;

    let mut robots: Vec<(Position, Position)> = Vec::new();

    for (_line_num, line) in contents.lines().enumerate() {
        let p: &str = line.split_ascii_whitespace().nth(0).unwrap().strip_prefix("p=").unwrap();
        let v: &str = line.split_ascii_whitespace().nth(1).unwrap().strip_prefix("v=").unwrap();

        let px: i64 = p.split(',').nth(0).unwrap().parse::<i64>().unwrap();
        let py: i64 = p.split(',').nth(1).unwrap().parse::<i64>().unwrap();

        let vx: i64 = v.split(',').nth(0).unwrap().parse::<i64>().unwrap();
        let vy: i64 = v.split(',').nth(1).unwrap().parse::<i64>().unwrap();

        robots.push(
            (Position::new(px as i32,py as i32), 
            Position::new(vx as i32, vy as i32))
        );
    }

    for i in 0..robots.len() {
        let (p, v) = robots.get_mut(i).unwrap();
        *p = *p + Position::new(v.x * seconds as i32, v.y * seconds as i32);
        p.x = p.x.rem_euclid(width as i32);
        p.y = p.y.rem_euclid(height as i32);
    }

    let (mut q1, mut q2, mut q3, mut q4): (u64, u64, u64, u64) = (0, 0, 0, 0);
    for (p, _) in robots {
        if p.x < (width as i32 - 1) / 2 && p.y < (height as i32 - 1) / 2 {
            // q1
            q1 += 1;
        } else if p.x > (width as i32 - 1) / 2 && p.y < (height as i32 - 1) / 2 {
            // q2
            q2 += 1;
        } else if p.x < (width as i32 - 1) / 2 && p.y > (height as i32 - 1) / 2 {
            // q3
            q3 += 1;
        } else if p.x > (width as i32 - 1) / 2 && p.y > (height as i32 - 1) / 2 {
            // q4
            q4 += 1;
        }
    }

    let answer: u64 = q1*q2*q3*q4;
    return format!("{}", answer);
}



#[allow(non_snake_case)]
pub fn d14_part2(contents: String) -> String {
    let width: u64 = 101;
    let height: u64 = 103;

    let mut robots: Vec<(Position, Position)> = Vec::new();
    let mut robots_len: usize = 0;

    for (_line_num, line) in contents.lines().enumerate() {
        let p: &str = line.split_ascii_whitespace().nth(0).unwrap().strip_prefix("p=").unwrap();
        let v: &str = line.split_ascii_whitespace().nth(1).unwrap().strip_prefix("v=").unwrap();

        let px: i64 = p.split(',').nth(0).unwrap().parse::<i64>().unwrap();
        let py: i64 = p.split(',').nth(1).unwrap().parse::<i64>().unwrap();

        let vx: i64 = v.split(',').nth(0).unwrap().parse::<i64>().unwrap();
        let vy: i64 = v.split(',').nth(1).unwrap().parse::<i64>().unwrap();

        robots.push(
            (Position::new(px as i32,py as i32), 
            Position::new(vx as i32, vy as i32))
        );
        robots_len += 1;
    }

    let mut count: u64 = 0;
    let mut least_danger: u64 = std::u64::MAX;
    loop {
        let mut found_tree: bool = false;

        // move the robots 1 second
        for i in 0..robots_len {
            let (p, v) = robots.get_mut(i).unwrap();
            *p = *p + Position::new(v.x as i32, v.y as i32);
            p.x = p.x.rem_euclid(width as i32);
            p.y = p.y.rem_euclid(height as i32);
        }

        count += 1;

        // calculate the danger
        let (mut q1, mut q2, mut q3, mut q4): (u64, u64, u64, u64) = (0, 0, 0, 0);
        for (p, _) in robots.clone() {
            if p.x < (width as i32 - 1) / 2 && p.y < (height as i32 - 1) / 2 {
                // q1
                q1 += 1;
            } else if p.x > (width as i32 - 1) / 2 && p.y < (height as i32 - 1) / 2 {
                // q2
                q2 += 1;
            } else if p.x < (width as i32 - 1) / 2 && p.y > (height as i32 - 1) / 2 {
                // q3
                q3 += 1;
            } else if p.x > (width as i32 - 1) / 2 && p.y > (height as i32 - 1) / 2 {
                // q4
                q4 += 1;
            }
        }

        // check the safest positions
        let danger: u64 = q1*q2*q3*q4;
        if danger < least_danger { // christmas tree will be the least dangerous?
            least_danger = danger;

            for (p, _) in robots.clone() {
                let mut adjacent_count: u64 = 0;
                for i in 1..=10 {
                    let adjacent: Position = p+Position::new(i, 0);
                    for (other_robot, _) in robots.clone() {
                        if other_robot != p && other_robot == adjacent {
                            adjacent_count += 1;
                        }
                    }
                }

                if adjacent_count > 8 {
                    found_tree = true;
                    break;
                }
            }
        }

        if found_tree || count > 1_000_000 {
            break;
        }
    }

    return format!("{}", count);
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
pub fn d15_part1(contents: String) -> String {
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
    return format!("{}", answer);
}

#[allow(non_snake_case)]
pub fn d15_part2(contents: String) -> String {
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
    }

    let mut answer: u64 = 0;
    for (row_num, row) in map.iter().enumerate() {
        for (col_num, char) in row.iter().enumerate() {
            if *char == '[' {
                answer += 100 * row_num as u64 + col_num as u64;
            }
        }
    }

    return format!("{}", answer);
}

#[allow(non_snake_case)]
fn day16_optimized_dijkstras_search(  weighted_map: &Vec<Vec<u8>>, start: Position, 
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
                                            day16_optimized_heuristic(
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
fn day16_optimized_heuristic(neighbor: Position, current: Position, previous: Position) -> f32 {
    let step_cost: f32 = (((neighbor.x - current.x) + (neighbor.y - current.y)) as f32).abs();
    let turn_cost: f32 = if (previous.x - neighbor.x).abs() == 1 && 
                            (previous.y - neighbor.y).abs() == 1 { 1000.0 } // turning 
                        else if previous == neighbor { 2000.0 } // did a 180
                        else { 0.0 }; // straight

    return step_cost + turn_cost;
}


#[allow(non_snake_case)]
pub fn d16_part1(contents: String) -> String {
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

    return format!("{}", day16_optimized_dijkstras_search(&map, start, end).unwrap_or(0));
}


#[allow(non_snake_case)]
fn day16_optimized_dijkstras_search_p2(weighted_map: &Vec<Vec<u8>>, start: Position, goal: Position) -> Option<Vec<Vec<Position>>> {
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
                                            day16_optimized_heuristic(
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
pub fn d16_part2(contents: String) -> String {
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

    let paths: Option<Vec<Vec<Position>>> = day16_optimized_dijkstras_search_p2(&map, start, end);
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

    return format!("{}", path_positions.len());
}