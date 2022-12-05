/// https://adventofcode.com/2022/day/3
use crate::file;

pub fn rucksack_priorities() {
    let mut priority = 0;
    if let Ok(data) = file::read_lines("data/d3a.txt") {
        for line in data {
            if let Ok(sacks) = line {
                let contents = sacks.split_at(sacks.len()/2);
                let s1: Vec<char> = contents.0.chars().collect();
                let s2: Vec<char> = contents.1.chars().collect();
                for c in s1 {
                    if s2.contains(&c) {
                        priority += priority_of(c);
                        break;
                    }
                }
            }
        }
    }
    println!("Priority: {}", priority);
}

pub fn rucksack_badges() {
    if let Ok(data) = file::read_lines("data/d3a.txt") {
        // read 3 lines at a time.
        let mut e1 = Vec::new();
        let mut e2 = Vec::new();
        let mut e3: Vec<char>;
        
        let mut cnt = 0;
        let mut total = 0;
        for line in data {
            match line {
                Ok(line) => {
                    if cnt == 0 {
                        e1 = line.chars().collect();
                        cnt += 1;
                    } else if cnt == 1 {
                        e2 = line.chars().collect();
                        cnt += 1;
                    } else {
                        e3 = line.chars().collect();
                        let common = find_common(
                            &find_common(&e1, &e2),
                            &find_common(&e1, &e3)
                        );
                        total += priority_of(common[0]);
                        cnt = 0;
                    }
                },
                _ => {},
            }
        }
        println!("Total priority: {}", total);
    }
}

fn find_common(g1: &Vec<char>, g2: &Vec<char>) -> Vec<char> {
    let mut common = Vec::new();
    for c in g1 {
        if g2.contains(c) {
            if !common.contains(c) {
                common.push(c.to_owned());
            }
        }
    }
    
    common
}

fn priority_of(c: char) -> i32 {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    
    let mut priority = 1;
    for c2 in &alphabet {
        if c == *c2 {
            break;
        }
        priority += 1;
    }
    priority
}