use std::fs;
use std::time::Instant;

const BUTTON_A_COST: f64 = 3.0;
const BUTTON_B_COST: f64 = 1.0;

#[allow(non_snake_case)]
fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");
    let mut now: Instant;
    let mut elapsed: std::time::Duration;

    now = Instant::now();
    let part1: u64 = part1(contents.clone());
    elapsed = now.elapsed();

    println!("part 1: {} ({:.2?})", part1, elapsed);

    now = Instant::now();
    let part2: u64 = part2(contents.clone(), 10_000_000_000_000); 
    elapsed = now.elapsed();

    println!("part 2: {} ({:.2?})", part2, elapsed);
}



#[allow(non_snake_case)]
fn part1(contents: String) -> u64 {
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

    return answer;
}



#[allow(non_snake_case)]
fn part2(contents: String, offset: u64) -> u64 {
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
                                    .parse::<u64>().unwrap() + offset;
            let prize_y = raw_equation.split(',').nth(1).unwrap()
                                    .strip_prefix(" Y=").unwrap()
                                    .parse::<u64>().unwrap() + offset;

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

    return answer;
}


#[cfg(test)] #[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 480);
    }

    #[test]
    fn test_part2() {
        // part 2 doesn't have any tests... :(
        let contents: String = fs::read_to_string("src/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone(), 0), 480);
    }
}