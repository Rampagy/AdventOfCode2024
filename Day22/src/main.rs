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
    let mut sequence_price_map: HashMap<Vec<i8>, Vec<u8>> = HashMap::new();

    for (_row_num, line) in contents.lines().enumerate() {
        let mut secret: u64 = line.parse::<u64>().unwrap();
        let mut prev_secret: u64 = secret;
        let mut delta_prices: Vec<i8> = Vec::new();
        let mut potential_changes: HashSet<Vec<i8>> = HashSet::new();

        for i in 1..iterations {
            secret = calc_next_secret(secret);
            delta_prices.push((secret%10) as i8 - (prev_secret%10) as i8);

            if i >= SEQUENCE_LENGTH {
                // start populating potential_changes
                let seq: Vec<i8> = delta_prices[(i-SEQUENCE_LENGTH)..].to_vec();

                // only add price to sequence_price_map if it's the first occurance of the sequence for this buyer
                if !potential_changes.contains(&seq.clone()) {
                    potential_changes.insert(seq.clone());
                    let price: u8 = (secret%10) as u8;
                    sequence_price_map.entry(seq.clone())
                                        .and_modify(|x| x.push(price))
                                        .or_insert(vec![price]);
                }
            }

            prev_secret = secret;
        }
    }

    // loop through each potential_change on all of the buyers and see which one produces the most money
    let mut max_money: u64 = 0;
    for (_sequence, prices) in sequence_price_map.iter() {
        let sequence_price: u64 = prices.iter().map(|x| *x as u64).sum();

        if sequence_price > max_money {
            max_money = sequence_price;
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