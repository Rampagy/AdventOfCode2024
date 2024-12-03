use std::fs;

#[allow(non_snake_case)]
fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    println!("part 1: {}", part1(contents.clone()));
    println!("part 2: {}", part2(contents.clone()));
}

#[allow(non_snake_case)]
fn part1(contents: String) -> u64 {
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

    return answer;
}

#[allow(non_snake_case)]
fn part2(contents: String) -> u64 {
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

    return answer;
}


#[cfg(test)] #[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 161);
    }

    #[test]
    fn test_part2() {
        let contents: String = fs::read_to_string("src/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), 48);
    }
}