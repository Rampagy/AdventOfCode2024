use std::collections::{HashSet, HashMap};
use std::{fs, u64};
use std::time::Instant;


const ITERATIONS: usize = 2000;
const SEQUENCE_LENGTH: usize = 4;


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
    let part2: u64 = part2(contents.clone(), ITERATIONS);
    elapsed = now.elapsed();

    println!("part 2: {} ({:.2?})", part2, elapsed);
}


#[allow(non_snake_case)]
fn calc_next_secret(mut secret: u64) -> u64 {

    // multiply by 64, xor with secret, then mod 16777216
    secret = (secret ^ (secret << 6)) % 16777216;

    // divide by 32, xor with secret, then mod 16777216
    secret = (secret ^ (secret >> 5)) % 16777216;

    // multiply by 2048, xor with secret, then mod 16777216
    secret = (secret ^ (secret << 11)) % 16777216;

    return secret;
}


#[allow(non_snake_case)]
fn part1(contents: String) -> u64 {
    let mut answer: u64 = 0;

    for (_row_num, line) in contents.lines().enumerate() {
        let mut secret: u64 = line.parse::<u64>().unwrap();

        for _ in 0..ITERATIONS {
            secret = calc_next_secret(secret);
        }
        answer += secret;
    }

    return answer;
}


#[allow(non_snake_case)]
fn part2(contents: String, iterations: usize) -> u64 {
    let mut potential_changes: HashSet<Vec<i8>> = HashSet::new();
    let mut buyer_sequence_price_maps: Vec<HashMap<Vec<i8>, u8>> = Vec::new();

    for (_row_num, line) in contents.lines().enumerate() {
        let mut secret: u64 = line.parse::<u64>().unwrap();
        let mut prev_secret: u64 = secret;
        let mut delta_prices: Vec<i8> = Vec::new();
        let mut sequence_price_map: HashMap<Vec<i8>, u8> = HashMap::new();

        for i in 1..iterations {
            secret = calc_next_secret(secret);
            delta_prices.push((secret%10) as i8 - (prev_secret%10) as i8);

            if i >= SEQUENCE_LENGTH {
                // start populating potential_changes
                let seq: Vec<i8> = delta_prices[(i-SEQUENCE_LENGTH)..].to_vec();
                potential_changes.insert(seq.clone());
                if !sequence_price_map.contains_key(&seq.clone()) {
                    sequence_price_map.insert(seq.clone(), (secret%10) as u8);
                }
            }

            prev_secret = secret;
        }

        // store the sequence to price map
        buyer_sequence_price_maps.push(sequence_price_map);
    }

    // loop through each potential_change on all of the buyers and see which one produces the most money
    let mut max_money: u64 = 0;
    for potential in potential_changes {
        let mut price_count: u64 = 0;
        for (_buyer_idx, buyer_sequence_price_map) in buyer_sequence_price_maps.iter().enumerate() {
            if let Some(price) = buyer_sequence_price_map.get(&potential) {
                price_count += *price as u64;
            }

            if (price_count as usize + (buyer_sequence_price_maps.len().saturating_sub(_buyer_idx+1) * 9)) <= max_money as usize {
                // if all the remaining indices were at max price it can't be greater than max_money
                // therefore there is no point to continue searching this buyer or any of the rest
                break;
            }
        }

        if price_count > max_money {
            max_money = price_count;
        }
    }

    return max_money;
}


#[cfg(test)] #[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test_nextsecret() {
        let answers: [u64; 10] = [15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432, 5908254];
        let mut secret: u64 = 123;
        for i in 0..10 {
            secret = calc_next_secret(secret);
            assert_eq!(answers[i], secret);
        }
    }

    #[test]
    fn test_nextsecret_and_getones() {
        let answers: [u64; 10] = [0, 6, 5, 4, 4, 6, 4, 4, 2, 4];
        let mut secret: u64 = 123;
        for i in 0..10 {
            secret = calc_next_secret(secret);
            assert_eq!(answers[i], secret%10);
        }
    }

    #[test]
    fn test_part1a() {
        let contents: String = fs::read_to_string("src/test1a.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 37327623);
    }

    #[test]
    fn test_part2a() {
        let contents: String = fs::read_to_string("src/test2a.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone(), ITERATIONS), 23);
    }

    #[test]
    fn test_part2b() {
        let contents: String = fs::read_to_string("src/test2b.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone(), 10), 6);
    }
}