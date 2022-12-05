/// https://adventofcode.com/2022/day/4
use core::fmt;

use crate::file;

#[derive(Debug)]
struct SectionRange {
    start: u32,
    end: u32,
}

impl fmt::Display for SectionRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut out = String::new();
        for n in 1u32..100 {
            if n >= self.start && n <= self.end {
                out.push('*');
            } else {
                out.push('.');
            }

        }
        write!(f, "{:3},{:3}: {}", self.start, self.end, out)
    }
}

impl From<&str> for SectionRange {
    fn from(s: &str) -> Self {
        let r = s.split_once("-").unwrap();
        SectionRange {
            start: r.0.parse::<u32>().unwrap(),
            end: r.1.parse::<u32>().unwrap()
        }
    }
}

pub fn count_section_ranges_encased_in_other() {
    let mut full_overlap_cnt = 0;
    let data = file::read_lines("data/d4a.txt").unwrap();
    for line in data {
        if let Ok(line) = line {
            let ranges = line.split_once(",").unwrap();
            let r1 = SectionRange::from(ranges.0);
            let r2 = SectionRange::from(ranges.1);
            if r1.start <= r2.start {
                // range 1 starts at or before range 2
                if r2.end <= r1.end || (r1.start == r2.start && r1.end <= r2.end) {
                    // range 2 ends at or before range 1 ends
                    full_overlap_cnt += 1;
                    // println!("{:?} is fully enclosed in {:?}", r2, r1)
                } else {
                    // println!("{:?} and {:?} do not contain a complete overlap of ranges (r1)", r1, r2);
                    // println!("{}", r1);
                    // println!("{}", r2);
                    // print_overlap(r1, r2);
                }
            } else {
                // range 1 starts at or after range 2
                if r1.end <= r2.end {
                    // range 1 ends at or before 2 does
                    full_overlap_cnt += 1;
                    // println!("{:?} is fully enclosed in {:?}", r1, r2)
                } else {
                    // println!("{:?} and {:?} do not contain a complete overlap of ranges", r1, r2);
                    // println!("{}", r1);
                    // println!("{}", r2);
                    // print_overlap(r1, r2);
                }
            }
        }
    }

    println!("Found {} pairs that have 1 range full enclosed in the other", full_overlap_cnt);
}

pub fn count_section_ranges_with_partial_overlap() {
    let mut overlap_cnt = 0;
    let data = file::read_lines("data/d4a.txt").unwrap();
    for line in data {
        if let Ok(line) = line {
            let ranges = line.split_once(",").unwrap();
            let r1 = str_to_i32_range(ranges.0.split_once("-").unwrap());
            let r2 = str_to_i32_range(ranges.1.split_once("-").unwrap());
            if r1.0 <= r2.0 {
                // range 1 starts at or before range 2
                if r1.1 >= r2.0 {
                    // range 1 ends anywhere after range 2 starts
                    overlap_cnt += 1;
                } 
            } else {
                // range 1 starts after range 2
                if r1.0 <= r2.1 {
                    // range 1 starts before range 2 ends
                    overlap_cnt += 1;
                }
            }
        }
    }

    println!("Found {} pairs that have 1 range with any overlap of the other", overlap_cnt);
}

fn str_to_i32_range(range: (&str, &str)) -> (u32, u32) {
    (range.0.parse::<u32>().unwrap(), range.1.parse::<u32>().unwrap())
}
