use std::fs;
use std::collections::HashMap;
use std::cmp::Ordering;

#[allow(non_snake_case)]
fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    println!("part 1: {}", part1(contents.clone()));
    println!("part 2: {}", part2(contents.clone()));
}


#[allow(non_snake_case)]
fn part1(contents: String) -> u64 {
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

    return answer;
}

#[allow(non_camel_case_types)]
struct custom_comparator {
    page_ordering_rules: HashMap<u64, Vec<u64>>
}

impl custom_comparator {
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
fn part2(contents: String) -> u64 {
    let mut answer: u64 = 0;
    let mut in_rules_section: bool = true;
    let mut comparator: custom_comparator = custom_comparator { 
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

    return answer;
}


#[cfg(test)] #[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 143);
    }

    #[test]
    fn test_part2() {
        let contents: String = fs::read_to_string("src/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), 123);
    }
}