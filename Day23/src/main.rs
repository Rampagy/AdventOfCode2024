use std::fs;
use std::time::Instant;
use std::collections::{HashSet, HashMap};


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
fn part1(contents: String) -> u64 {
    let mut answer: u64 = 0;
    let mut networks: HashMap<String, HashSet<String>> = HashMap::default();

    for (_row_num, line) in contents.lines().enumerate() {
        let nodes: Vec<String> = line.split('-')
                                    .map(|x| x.to_string())
                                    .collect();

        if let Some(network) = networks.get_mut(nodes.first().unwrap()) {
            // if the first node is in the network, insert the last as a connection to the first
            network.insert(nodes.last().unwrap().to_string());

            if let Some(network1) = networks.get_mut(nodes.last().unwrap()) {
                // if the last node is in the network, insert the first as a connection to the last
                network1.insert(nodes.first().unwrap().to_string());
            }
        }

        if let Some(network) = networks.get_mut(nodes.last().unwrap()) {
            // if the last node is in the network, insert the first as a connection to the last
            network.insert(nodes.first().unwrap().to_string());

            if let Some(network1) = networks.get_mut(nodes.first().unwrap()) {
                // if the last node is in the network, insert the first as a connection to the last
                network1.insert(nodes.last().unwrap().to_string());
            }
        }

        if !networks.contains_key(nodes.first().unwrap()) {
            let mut last_set:  HashSet<String> = HashSet::new();
            last_set.insert(nodes.last().unwrap().to_string());
            networks.insert(nodes.first().unwrap().to_string(), last_set);
        }

        if !networks.contains_key(nodes.last().unwrap()) {
            let mut first_set: HashSet<String> = HashSet::new();
            first_set.insert(nodes.first().unwrap().to_string());
            networks.insert(nodes.last().unwrap().to_string(), first_set);
        }
    }

    for (parent, children) in networks.iter() {
        if children.len() == 2 {
            if parent.starts_with('t') {
                answer += 1;
            } else {
                // check if the children start with a 't' if the parent doesn't
                for child in children {
                    if child.starts_with('t') {
                        answer += 1;
                        break;
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

    for (_row_num, line) in contents.lines().enumerate() {
        
    }

    return answer;
}


#[cfg(test)] #[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test_part1a() {
        let contents: String = fs::read_to_string("src/test1a.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 7);
    }

    #[test]
    fn test_part2a() {
        let contents: String = fs::read_to_string("src/test2a.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), 23);
    }
}