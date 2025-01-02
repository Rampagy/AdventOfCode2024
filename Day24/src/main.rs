use std::fs;
use std::time::Instant;
use std::collections::{HashMap, VecDeque};
use std::fmt;


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
    let part2: String = part2(contents.clone(), 4);
    elapsed = now.elapsed();

    println!("part 2: {} ({:.2?})", part2, elapsed);
}


#[derive(PartialEq, Eq, Hash, Clone)]
struct Operation {
    pub operand1: String,
    pub operand2: String,
    pub operation: String,
    pub result: String,
}


impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {} -> {}", self.operand1, self.operation, self.operand2, self.result)
    }
}


#[allow(dead_code)]
impl Operation {
    // Constructor
    pub fn new(operand1: String, operand2: String, operation: String, result: String) -> Self {
        Self { operand1: operand1, operand2: operand2, operation: operation, result: result }
    }

    pub fn exe_cmd(&self, operand1_val: u8, operand2_val: u8) -> u8 {
        match self.operation.as_str() {
            "AND" => operand1_val & operand2_val,
            "XOR" => operand1_val ^ operand2_val,
            "OR" => operand1_val | operand2_val,
            _ => panic!("unrecognized operation"),
        }
    }
}


#[allow(non_snake_case)]
fn part1(contents: String) -> u64 {
    let mut get_inputs: bool = true;
    let mut inputs: HashMap<String, u8> = HashMap::default();
    let mut operations: VecDeque<Operation> = VecDeque::new();

    for (_row_num, line) in contents.lines().enumerate() {
        if line == "" {
            get_inputs = false;
        } else if get_inputs {
            let variable: Vec<String> = line.split(": ").map(|x| x.to_string()).collect();
            inputs.insert(variable.first().unwrap().to_string(), variable.last().unwrap().parse::<u8>().unwrap());
        } else {
            // collect operations
            let raw_operation: Vec<String> = line.split_ascii_whitespace().map(|x| x.to_string()).collect();
            operations.push_back(
                    Operation::new(
                        raw_operation[0].clone(), 
                        raw_operation[2].clone(), 
                        raw_operation[1].clone(), 
                        raw_operation[4].clone())
                    );
        }
    }


    while !operations.is_empty() {
        let operation: Operation = operations.pop_front().unwrap();
        if let Some(op1) = inputs.get(&operation.operand1) {
            if let Some(op2) = inputs.get(&operation.operand2) {
                // both operand were found
                inputs.insert(operation.result.clone(), operation.exe_cmd(*op1, *op2));

                // this operation is complete - don't add it back into the deque
            } else {
                // second operand was not found - put the oepration back in (at the end)
                operations.push_back(operation);
            }
        } else {
            // first operand was not found - put the oepration back in (at the end)
            operations.push_back(operation);
        }
    }

    // construct the value starting with z00
    let mut answer: u64 = 0;
    let mut current_bit: u8 = 0;
    let mut current_bit_as_str: String = format!("z{:0>2}", current_bit);
    while inputs.contains_key(current_bit_as_str.as_str()) {
        answer |= (*inputs.get(&current_bit_as_str).unwrap() as u64) << (current_bit as u64);

        current_bit += 1;
        current_bit_as_str = format!("z{:0>2}", current_bit);
    }

    return answer;
}


#[allow(non_snake_case)]
fn part2(contents: String, num_swapped: u64) -> String {
    for (_row_num, line) in contents.lines().enumerate() {

    }


    return "".to_string();
}


#[cfg(test)] #[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test_part1a() {
        let contents: String = fs::read_to_string("src/test1a.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 4);
    }

    #[test]
    fn test_part1b() {
        let contents: String = fs::read_to_string("src/test1b.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 2024);
    }

    #[test]
    fn test_part2a() {
        let contents: String = fs::read_to_string("src/test2a.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone(), 2), "z00,z01,z02,z05".to_string());
    }
}