use std::fs;
use std::time::Instant;


#[allow(non_snake_case)]
fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    let mut now: Instant = Instant::now();
    let part1: u64 = part1(contents.clone());
    let mut elapsed: std::time::Duration = now.elapsed();

    println!("part 1: {} ({:.2?})", part1, elapsed); // 6201390809186 - too high

    now = Instant::now();
    let part2: u64 = part2(contents.clone());
    elapsed = now.elapsed();

    println!("part 2: {} ({:.2?})", part2, elapsed);
}

#[allow(non_snake_case)]
fn part1(contents: String) -> u64 {
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

    return answer;
}


#[allow(non_snake_case)]
fn part2(contents: String) -> u64 {
    let mut answer: u64 = 0;


    for (_line_num, line) in contents.lines().enumerate() {

    }

    return answer;
}


#[cfg(test)] #[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 1928);
    }

    #[test]
    fn test_part1_custom0() {
        let contents: String = "2230103020404030402".to_string();
        assert_eq!(part1(contents.clone()), 2259);
    }

    #[test]
    fn test_part1_custom1() {
        let contents: String = "233133121414131402".to_string();
        assert_eq!(part1(contents.clone()), 198);
        // 2 3  3  13  3  12 14   14   13  14   02
        // 00...111.222...3..4....5....6...7......
        // 0076511142223
        // 0*0 + 0*1 + 7*2 + 6*3 + 5*4 + 1*5 + 1*6 + 1*7 + 4*8 + 2*9 + 2*10 + 2*11 + 3*12
        // 198
    }

    #[test]
    fn test_part2() {
        let contents: String = fs::read_to_string("src/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), 34);
    }
}