/// 
use crate::file;
use core::fmt;

struct Crate {
    name: String,
}

impl From<char> for Crate {
    fn from(c: char) -> Self {
       Crate {
         name: String::from(c)
       } 
    }
}

struct Stacks {
    stacks: Vec<Vec<Crate>>,
}

struct Instruction {
    quantity: i32,
    source: usize,
    destination: usize,
}

impl From<String> for Instruction {
    fn from(s: String) -> Self {
        let mut iter = s.split_whitespace();
        iter.next(); // skip move
        let qty = iter.next().unwrap();
        iter.next(); // skip from
        let src = iter.next().unwrap();
        iter.next(); // skip to
        let dest = iter.next().unwrap();

        Instruction {
            quantity: qty.parse::<i32>().unwrap(),
            source: src.parse::<usize>().unwrap(),
            destination: dest.parse::<usize>().unwrap()
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "move {} from {}  to {}", self.quantity, self.source, self.destination)
    }
}

pub fn determine_final_stack_states() {
    let data = file::read_lines("data/d5a.txt").unwrap();
    let mut initialized = false;
    let mut raw_stacks = Vec::new();
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for line in data {
        if let Ok(line) = line {
            if !initialized && line.len() == 0 {
                initialized = true;
                stacks = process_stacks(&mut raw_stacks);
                continue;
            }
            if !initialized {
                raw_stacks.push(line);
            } else {
                // parse instruction
                // process move
                // move qty from src to dest
                let mut instruction = line.split_whitespace();
                instruction.next();
                let qty = instruction.next().unwrap().parse::<i32>().unwrap();
                instruction.next();
                let src = instruction.next().unwrap().parse::<usize>().unwrap();
                instruction.next();
                let dest = instruction.next().unwrap().parse::<usize>().unwrap();
                
                for _ in 1..=qty {
                    let c = stacks[src].pop().unwrap();
                    stacks[dest].push(c);
                }
            }

        }
    }
    let mut final_state = String::new();
    for mut stack in stacks {
        if let Some(c) = stack.pop() {
            final_state.push(c);
        }
    }
    println!("{}", final_state);
}

pub fn determine_final_stack_states_9001() {
    let data = file::read_lines("data/d5a.txt").unwrap();
    let mut initialized = false;
    let mut raw_stacks = Vec::new();
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut temp = Vec::new();
    for line in data {
        if let Ok(line) = line {
            if !initialized && line.len() == 0 {
                initialized = true;
                stacks = process_stacks(&mut raw_stacks);
                continue;
            }
            if !initialized {
                raw_stacks.push(line);
            } else {
                // parse instruction
                // process move
                // move qty from src to dest
                let mut instruction = line.split_whitespace();
                instruction.next();
                let qty = instruction.next().unwrap().parse::<i32>().unwrap();
                instruction.next();
                let src = instruction.next().unwrap().parse::<usize>().unwrap();
                instruction.next();
                let dest = instruction.next().unwrap().parse::<usize>().unwrap();
                
                for _ in 1..=qty {
                    let c = stacks[src].pop().unwrap();
                    temp.push(c);
                }
                for _ in 1..=qty {
                    let c = temp.pop().unwrap();
                    stacks[dest].push(c);
                }
            }

        }
    }
    let mut final_state = String::new();
    for mut stack in stacks {
        if let Some(c) = stack.pop() {
            final_state.push(c);
        }
    }
    println!("{}", final_state);
}

fn process_stacks(raw: &mut Vec<String>) -> Vec<Vec<char>> {
    let stack_nums = raw.pop().unwrap();
    let mut stacks = Vec::new();
    stacks.push(Vec::new()); // occupy index 0
    for s in stack_nums.split_whitespace() {
        stacks.push(Vec::new());
    }

    raw.reverse();
    for line in raw {
        let mut i = 0;
        for c in line.chars() {
            if i % 4 == 1 {
                if c != ' ' {
                    let stack= ((i-1)/4)+1;
                    stacks[stack].push(c);
                }
            }
            i += 1;
        }
    }
    stacks
}

fn print_stacks(stacks: &Vec<Vec<char>>) {
    for stack in stacks {
        println!("{:?}", stack);
    }
}

pub fn crate_mover_results() {
    let data = file::read_lines("data/d5s.txt").unwrap();
    let mut initialized = false;
    let mut init_lines = Vec::new();
    let mut stacks: Vec<Vec<Crate>> = Vec::new();
    for line in data {
        if let Ok(line) = line {
            if !initialized {
                if line.len() == 0 {
                    stacks = initialize_stacks(&mut init_lines);
                    initialized = true;
                } else {
                    init_lines.push(line);
                }
            } else {
                move_crates_single(line, &mut stacks);
            }
        }
    }
    
}

fn move_crates_single(line: String, stacks: &mut Vec<Vec<Crate>>) {
    let instruction = Instruction::from(line);
    for _ in 1..=instruction.quantity {
        let c = stacks[instruction.source].pop().unwrap();
        stacks[instruction.destination].push(c);
    }
}

fn move_crates_multiple(line: String, stacks: &mut Vec<Vec<Crate>>) {
    let instruction = Instruction::from(line);
    let mut temp = Vec::new();
    for _ in 1..=instruction.quantity {
        let c = stacks[instruction.source].pop().unwrap();
        temp.push(c);
    }
    for _ in 1..=instruction.quantity {
        let c = temp.pop().unwrap();
        stacks[instruction.destination].push(c);
    }
}

fn initialize_stacks(raw: &mut Vec<String>) -> Vec<Vec<Crate>> {
    raw.pop(); // We don't care about names
    raw.reverse(); // We want to start from the bottom up so stacks look right.
    let mut stacks = Vec::new();
    stacks.push(Vec::new());

    for line in raw {
        let mut i = 0;
        for c in line.chars() {
            if i % 4 == 1 {
                // We want to look at the middle item of each group, which will either be a letter, or whitespace
                if c != ' ' {
                    let stack = ((i-1)/4)+1;
                    stacks[stack].push(Crate::from(c))
                }
            }
            i += 1;
        }
    }
    stacks
}