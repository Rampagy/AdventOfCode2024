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

    return answer;
}

#[allow(non_snake_case)]
fn part2(contents: String) -> u64 {
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

    return answer;
}


#[cfg(test)] #[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 18);
    }

    #[test]
    fn test_part2() {
        let contents: String = fs::read_to_string("src/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), 9);
    }
}