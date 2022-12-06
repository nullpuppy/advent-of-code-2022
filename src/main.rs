mod day;
mod file;

use day::day1 as d1;
use day::day2 as d2;
use day::day3 as d3;
use day::day4 as d4;
use day::day5 as d5;
use day::day6 as d6;

fn main() {
    println!("Advent of Code 2022!");
    println!("Day 1: Calorie Counting");
    d1::highest_calorie_count_on_elf();
    d1::total_calories_in_top_three();
    println!("Day 2");
    d2::score_tournament_strategy_given_hand();
    d2::score_tournament_strategy_given_outcome();
    println!("Day 3");
    d3::rucksack_priorities();
    d3::rucksack_badges();
    println!("Day 4");
    d4::count_section_ranges_encased_in_other();
    d4::count_section_ranges_with_partial_overlap();
    println!("Day 5");
    d5::determine_final_stack_states();
    d5::determine_final_stack_states_9001();
    println!("Day 6");
    d6::task1();
    d6::task2();
}