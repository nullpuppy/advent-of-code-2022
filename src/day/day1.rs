/// https://adventofcode.com/2022/day/1
use crate::file;


/// Find highest total calorie count of snacks carried by a single elf 
pub fn highest_calorie_count_on_elf() {
    let mut highest_cal = 0;
    let mut highest_elf = 0;
    let mut cur_elf = 1;
    let mut cur_cals = 0;

    let data = file::read_lines("data/d1a.txt").unwrap();
    for line in data {
        if let Ok(cals) = line {
            if let Ok(num) = cals.parse::<i32>() {
                cur_cals += num;
            } else {
                if cur_cals > highest_cal {
                    highest_cal = cur_cals;
                    highest_elf = cur_elf;
                }

                cur_cals = 0;
                cur_elf += 1;
            }
        }
    }
   
    println!("Highest cal value found {} by {}", highest_cal, highest_elf);
}

/// Find total calorie count carried by the top 3
pub fn total_calories_in_top_three() {
    let mut cal_counts = Vec::new();
    let mut cur_cals = 0;

    if let Ok(data) = file::read_lines("data/d1a.txt") {
        for line in data {
            if let Ok(cals) = line {
                if let Ok(num) = cals.parse::<i32>() {
                    cur_cals += num;
                } else {
                    cal_counts.push(cur_cals);
                    cur_cals = 0;
                }
            }
        }
        if cur_cals > 0 {
            cal_counts.push(cur_cals);
        }
    }
    cal_counts.sort();
    let len = cal_counts.len();
    println!("len? {}", len);
    println!("Top 3 are {}, {}, {} with a total of {}",
        cal_counts[len-1],
        cal_counts[len-2],
        cal_counts[len-3],
        cal_counts[len-1] + cal_counts[len-2] + cal_counts[len-3]
    );
}
