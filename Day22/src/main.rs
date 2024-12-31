use std::fs;
use std::time::Instant;


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

        for _ in 0..2000 {
            secret = calc_next_secret(secret);
        }
        answer += secret;
    }

    return answer;
}


#[allow(non_snake_case)]
fn part2(contents: String) -> u64 {
    let mut answer: u64 = 0;

    for (_row_num, line) in contents.lines().enumerate() {

    }

    return answer;
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
    fn test_part1a() {
        let contents: String = fs::read_to_string("src/test1a.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 37327623);
    }

    #[test]
    fn test_part2a() {
        let contents: String = fs::read_to_string("src/test2a.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), 126384);
    }
}