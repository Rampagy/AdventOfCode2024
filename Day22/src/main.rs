use std::collections::HashSet;
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
    let part2: u64 = part2(contents.clone());
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
fn part2(contents: String) -> u64 {
    let mut secrets: Vec<[u64; ITERATIONS]> = Vec::new();
    let mut potential_changes: HashSet<Vec<i8>> = HashSet::new();
    let mut buyer_deltas_prices: Vec<Vec<i8>> = Vec::new();

    for (_row_num, line) in contents.lines().enumerate() {
        let mut secret: u64 = line.parse::<u64>().unwrap();
        let mut delta_prices: Vec<i8> = Vec::new();
        let mut buyer_secrets: [u64; ITERATIONS] = [0; ITERATIONS];
        buyer_secrets[0] = secret;

        for i in 1..ITERATIONS {
            secret = calc_next_secret(secret);
            buyer_secrets[i] = secret;
            delta_prices.push((secret%10) as i8 - (buyer_secrets[i-1]%10) as i8);

            if i >= SEQUENCE_LENGTH {
                // start populating potential_changes
                potential_changes.insert(delta_prices[(i-SEQUENCE_LENGTH)..].to_vec());
            }
        }

        // store the original buyer secrets
        secrets.push(buyer_secrets);

        // store the buyer deltas
        buyer_deltas_prices.push(delta_prices);
    }

    // loop through each potential_change on all of the buyers and see which one produces the most money
    let mut max_money: u64 = 0;
    for potential in potential_changes {
        let mut price_count: u64 = 0;
        for (buyer_idx, buyer_price) in buyer_deltas_prices.clone().iter().enumerate() {
            for i in SEQUENCE_LENGTH..ITERATIONS {
                if potential[0] == buyer_price[i-4] && 
                    potential[1] == buyer_price[i-3] && 
                    potential[2] == buyer_price[i-2] && 
                    potential[3] == buyer_price[i-1] {
                        // add the current bananas
                        price_count += secrets[buyer_idx][i]%10;

                        // skip the rest of the iterations for this buyer
                        break;
                }
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
        assert_eq!(part2(contents.clone()), 23);
    }
}