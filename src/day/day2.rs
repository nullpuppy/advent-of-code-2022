/// https://adventofcode.com/2022/day/2
use crate::file;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
    Invalid,
}

enum Outcome {
    Win,
    Lose,
    Draw,
    Invalid,
}

pub fn score_tournament_strategy_given_hand() {
    let mut total_score = 0;
    if let Ok(data) = file::read_lines("data/d2a.txt") {
        for line in data {
            if let Ok(pair) = line {
                let mut strategy = pair.split_whitespace();
                let opp_play = match strategy.next() {
                    Some("A") => Hand::Rock,
                    Some("B") => Hand::Paper,
                    Some("C") => Hand::Scissors,
                    _ => Hand::Invalid,
                };
                let my_play = match strategy.next() {
                    Some("X") => Hand::Rock,
                    Some("Y") => Hand::Paper,
                    Some("Z") => Hand::Scissors,
                    _ => Hand::Invalid,
                };
                // split line an whitespace
                // first is opponent, second is choice to use
                let score = score_round(&opp_play, &my_play);
                total_score += score;
                // println!("{:?} vs {:?}: {} {}", opp_play, my_play, score, total_score);
            }
        }
        println!("Tournament Score: {}", total_score);
    }
}

pub fn score_tournament_strategy_given_outcome() {
    // A/X Rock     1
    // B/Y Paper    2
    // C/Z Scissors 3
    let mut total_score = 0;
    if let Ok(data) = file::read_lines("data/d2a.txt") {
        for line in data {
            if let Ok(pair) = line {
                let mut strategy = pair.split_whitespace();
                let opp_play = match strategy.next() {
                    Some("A") => Hand::Rock,
                    Some("B") => Hand::Paper,
                    Some("C") => Hand::Scissors,
                    _ => Hand::Invalid,
                };
                let outcome = match strategy.next() {
                    Some("X") => Outcome::Lose,
                    Some("Y") => Outcome::Draw,
                    Some("Z") => Outcome::Win,
                    _ => Outcome::Invalid,
                };
                let my_play = determine_hand(&opp_play, &outcome);
                // split line an whitespace
                // first is opponent, second is choice to use
                let score = score_round(&opp_play, &my_play);
                total_score += score;
                // println!("{:?} vs {:?}: {} {}", opp_play, my_play, score, total_score);
            }
        }
        println!("Tournament Score: {}", total_score);
    }

}

fn determine_hand(h1: &Hand, outcome: &Outcome) -> Hand {
    match outcome {
        Outcome::Win => {
            match h1 {
                Hand::Rock => Hand::Paper,
                Hand::Paper => Hand::Scissors,
                Hand::Scissors => Hand::Rock,
                Hand::Invalid => Hand::Invalid,
            }
        },
        Outcome::Lose => {
            match h1 {
                Hand::Rock => Hand::Scissors,
                Hand::Paper => Hand::Rock,
                Hand::Scissors => Hand::Paper,
                Hand::Invalid => Hand::Invalid,
            }
        },
        Outcome::Draw => {
            Hand::from(*h1)
        },
        Outcome::Invalid => {
            Hand::Invalid
        }
    }
}

fn score_round(h1: &Hand, h2: &Hand) -> i32 {
    let mut score = match h2 {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissors => 3,
        Hand::Invalid => 0,
    };
    if h1 == h2 {
        score += 3;
    } else {
        score += match h1 {
            Hand::Rock => {
                match h2 {
                    Hand::Paper => 6,
                    _ => 0,
                }
            },
            Hand::Paper => {
                match h2 {
                    Hand::Scissors => 6,
                    _ => 0,
                }

            },
            Hand::Scissors => {
                match h2 {
                    Hand::Rock => 6,
                    _ => 0,
                }
            },
            Hand::Invalid => {
                0
            },
        }
    }

    score
}

