use std::collections::HashMap;
use std::cmp::Ordering;


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

