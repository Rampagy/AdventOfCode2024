use std::ops::Div;
use std::{fs, u64};
use std::time::Instant;


#[allow(non_snake_case)]
fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");
    let mut now: Instant;
    let mut elapsed: std::time::Duration;

    now = Instant::now();
    let part1: String = part1(contents.clone(), false, 0, false, 0, false, 0);
    elapsed = now.elapsed();

    println!("part 1: {} ({:.2?})", part1, elapsed);

    now = Instant::now();
    let part2: u64 = part2(contents.clone());
    elapsed = now.elapsed();

    println!("part 2: {} ({:.2?})", part2, elapsed);
}

#[derive(Clone, Debug)] #[allow(non_snake_case, non_camel_case_types)]
struct cpu_registers {
    pub RegA: u64,
    pub RegB: u64,
    pub RegC: u64,
    pub instructionpointer: usize,
    pub program: Vec<u64>,
    pub console: Vec<u64>,
}

#[allow(non_snake_case)]
impl cpu_registers {
    pub fn new() -> Self {
        Self { RegA: 0, RegB: 0, RegC: 0, instructionpointer: 0, program: vec![], console: vec![] }
    }

    pub fn execute_instruction(&mut self) -> () {
        let opcode: u64 = self.program[self.instructionpointer];
        let opval: u64 = self.program[self.instructionpointer+1];
        let mut skip_ip_increment: bool = false;

        if opcode == 0 {
            // adv instruction, integer division with truncation
            // numerator is A register, denominator is 2^combo value, gets saved to RegA
            self.RegA = self.RegA.div(1 << self.get_combo_value(opval));
        } else if opcode == 1 {
            // bxl instruction, bitwise XOR
            // regb xor opval saved to regb
            self.RegB = self.RegB ^ opval;
        } else if opcode == 2 {
            // bst isntruction, modulo (potentially bit mask 0x7)
            // combo value modulo 8 saved to regb
            self.RegB = self.get_combo_value(opval) % 8;
        } else if opcode == 3 {
            // jnz instruction
            // does nothing if RegA is 0, if non zero jump to opval
            if self.RegA != 0 {
                self.instructionpointer = opval as usize;
                skip_ip_increment = true;
            }
        } else if opcode == 4 {
            // bxc instruction, bitwise xor
            // regb xor regc saved to regb
            self.RegB = self.RegB ^ self.RegC;
        } else if opcode == 5 {
            // out instruction, print result
            // combo value modulo 8 and outputs the value (comma separated)
            self.console.push(self.get_combo_value(opval) % 8);
        } else if opcode == 6 {
            // bdv intstruction
            // same as adv, but result is is saved to RegB
            self.RegB = self.RegA.div(1 << self.get_combo_value(opval));
        } else if opcode == 7 {
            // cdv instruction
            // same as adv, but result is saved to RegB
            self.RegC = self.RegA.div(1 << self.get_combo_value(opval));
        }

        if !skip_ip_increment {
            self.instructionpointer += 2;
        }
    }

    fn get_combo_value(&self, value: u64) -> u64 {
        return match value {
            4 => self.RegA,
            5 => self.RegB,
            6 => self.RegC,
            _ => value, // everything else return the value (7 isn't valid, so not sure what else to do with it)
        }
    }
}


#[allow(non_snake_case)]
fn part1(contents: String, 
        overridea: bool, a: u64, 
        overrideb: bool, b: u64, 
        overridec:bool, c:u64) -> String {
    let mut cpu: cpu_registers = cpu_registers::new();

    for (_line_num, line) in contents.lines().enumerate() {
        if line.starts_with("Register A:") {
            cpu.RegA = line.strip_prefix("Register A: ").unwrap().parse::<u64>().unwrap();
        } else if line.starts_with("Register B:") {
            cpu.RegB = line.strip_prefix("Register B: ").unwrap().parse::<u64>().unwrap();
        } else if line.starts_with("Register C:") {
            cpu.RegC = line.strip_prefix("Register C: ").unwrap().parse::<u64>().unwrap();
        } else if line.starts_with("Program:") {
            cpu.program = line.strip_prefix("Program: ").unwrap().split(',').map(|x| x.parse::<u64>().unwrap()).collect();
        }
    }

    if overridea {
        cpu.RegA = a;
    }

    if overrideb {
        cpu.RegB = b;
    }

    if overridec {
        cpu.RegC = c;
    }

    // loop through the whole program
    while cpu.instructionpointer < cpu.program.len()-1 {
        cpu.execute_instruction();
    }

    return cpu.console.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");
}



#[allow(non_snake_case)]
fn part2(contents: String) -> u64 {
    let mut cpu: cpu_registers = cpu_registers::new();

    for (_line_num, line) in contents.lines().enumerate() {
        if line.starts_with("Register A:") {
            cpu.RegA = line.strip_prefix("Register A: ").unwrap().parse::<u64>().unwrap();
        } else if line.starts_with("Register B:") {
            cpu.RegB = line.strip_prefix("Register B: ").unwrap().parse::<u64>().unwrap();
        } else if line.starts_with("Register C:") {
            cpu.RegC = line.strip_prefix("Register C: ").unwrap().parse::<u64>().unwrap();
        } else if line.starts_with("Program:") {
            cpu.program = line.strip_prefix("Program: ").unwrap().split(',').map(|x| x.parse::<u64>().unwrap()).collect();
        }
    }

    // what this program does:
    // 2,4   b = a % 8
    // 1,2   b = b ^ 2
    // 7,5   c = a / (1 << b)
    // 1,3   b = b ^ 3
    // 4,3   b = b ^ c
    // 5,5   out = b % 8
    // 0,3   a = a / 8
    // 3,0   if a == 0 ? break : restart

    // what we know about a:
    // the console output has to be equal to the program
    // need to reverse engineer the program to find the initial value of register A
    // program always ends with jump to beginning of program if RegA is non-zero
    // so we know the program always ends with RegA being zero
    // astart value = 8^(length of program)
    // 35184372088832 <= a < 281474976710656
    // 100000100 and the bottom 4 octal digits add up to between 8 and 16?

    // what we know about b:
    // b has no effect because it gets overwritten (by a%8) before it is used

    // now what about c:
    // c has no effect because it gets overwritten (by a/(1<<b)) before it is used


    for i in (0o1000000000000000..0o1000000000001767 as u64).step_by(1) {
        let mut cpu_clone: cpu_registers = cpu.clone();
        cpu_clone.RegA = i;

        // loop through the whole program
        while cpu_clone.instructionpointer < cpu_clone.program.len()-1 {
            cpu_clone.execute_instruction();
        }

        println!("a:{:#o} {:#b}   {}", i, i, cpu_clone.console.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","));

        if cpu_clone.console == cpu.program {
            println!("a:{}", i);
            cpu = cpu_clone;
            break;
        }
    }

    return cpu.RegA;
}



#[cfg(test)] #[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test_part1a() {
        let contents: String = fs::read_to_string("src/test1a.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone(), false, 0, false, 0, false, 0), "4,6,3,5,6,3,5,2,1,0".to_string());
    }


    #[test]
    fn test_part2a() {
        let contents: String = fs::read_to_string("src/test2a.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), 117440);
    }
}