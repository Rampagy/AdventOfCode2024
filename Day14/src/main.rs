mod position;

use std::fs;
use std::time::Instant;
use position::Position;

#[allow(non_snake_case)]
fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");
    let mut now: Instant;
    let mut elapsed: std::time::Duration;

    now = Instant::now();
    let part1: u64 = part1(contents.clone(), 101, 103, 100);
    elapsed = now.elapsed();

    println!("part 1: {} ({:.2?})", part1, elapsed);

    now = Instant::now();
    let part2: u64 = part2(contents.clone(), 101, 103); 
    elapsed = now.elapsed();

    println!("part 2: {} ({:.2?})", part2, elapsed); // 777 too low, 778 too low
}



#[allow(non_snake_case)]
fn part1(contents: String, width: u64, height: u64, seconds: u64) -> u64 {
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
    return answer;
}



#[allow(non_snake_case)]
fn part2(contents: String, width: u64, height: u64) -> u64 {
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
            println!("danger: {}, checking... {}", danger, count);

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

    return count;
}


#[cfg(test)] #[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone(), 11, 7, 100), 12);
    }

    #[test]
    fn test_part2() {
        let contents: String = fs::read_to_string("src/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone(), 11, 7), 12);
    }
}