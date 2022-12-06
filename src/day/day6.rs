use std::collections::{VecDeque, HashSet};
use crate::file;

const PACKET_MARKER_MAGIC: usize = 4;
const MESSAGE_MARKER_MAGIC: usize = 14;

pub fn task1() {
    let data = file::read_lines("data/d6a.txt").unwrap();
    for line in data {
        if let Ok(line) = line {
            let marker_pos = get_marker_position(&line, PACKET_MARKER_MAGIC);
            println!("Packet Marker: {}", marker_pos);
        }
    }
}

pub fn task2() {
    let data = file::read_lines("data/d6a.txt").unwrap();
    for line in data {
        if let Ok(line) = line {
            let marker_pos = get_marker_position(&line, MESSAGE_MARKER_MAGIC);
            println!("Message Marker: {}", marker_pos);
        }
    }
}

fn get_marker_position(input: &str, cap: usize) -> usize {
    let mut count = 1;
    let mut recent = VecDeque::with_capacity(cap);
    for c in input.chars() {
        if count < cap - 1 {
            recent.push_back(c.to_owned());
        } else {
            recent.push_back(c);
            if recent.len() > cap {
                recent.pop_front();
            }

            let mut set = HashSet::new();
            for ch in &recent {
                set.insert(ch.to_owned());
            }
            if set.len() == cap {
                break;
            }
        }

        count += 1;
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::day::day6::{PACKET_MARKER_MAGIC, MESSAGE_MARKER_MAGIC};

    use super::get_marker_position;

    #[test]
    fn packet_marker_test() {
        let samples = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];
        for s in samples {
            let pos = get_marker_position(s.0, PACKET_MARKER_MAGIC);
            assert_eq!(pos, s.1);
        }
    }

    #[test]
    fn message_marker_test() {

        let samples = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];
        for s in samples {
            let pos = get_marker_position(s.0, MESSAGE_MARKER_MAGIC);
            assert_eq!(pos, s.1);
        }
    }
}