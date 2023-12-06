use std::fs;
use std::collections::{
    HashSet,
    HashMap,
};

use crate::Solver;

pub struct Day;

impl Solver<i32> for Day {
    fn run_01() -> i32 {
        let mut cumsum = 0;
    
        let parse_number = |line: &str, i: usize| {
            line.as_bytes()[i..i+2]
                .iter()
                .map(|&byte| {
                    let ch = byte as char;
                    if ch == ' ' {
                        '0'
                    } else {
                        ch
                    }
                })
                .collect::<String>()
                .parse::<i32>()
                .unwrap()
        };
    
        for line in fs::read_to_string("./src/d04/input.txt").unwrap().lines() {
            let mut i = line.bytes().position(|ch| ch as char == ':').unwrap() + 2;
            let mut winning_numbers = HashSet::new();
    
            while line.as_bytes()[i] as char != '|' {
                let winning_number = parse_number(line, i);
    
                winning_numbers.insert(winning_number);
    
                i += 3;
            }
    
            i += 2;
    
            let mut score = 1;
    
            while i < line.len() {
                let candidate_number = parse_number(line, i);
    
                if winning_numbers.contains(&candidate_number) {
                    score *= 2;
                }
    
                i += 3;
            }
    
            score /= 2;
            cumsum += score;
        }
    
        cumsum
    }
    
    fn run_02() -> i32 {
        let parse_number = |line: &str, i: usize, len: usize| {
            line.as_bytes()[i..i+len]
                .iter()
                .map(|&byte| {
                    let ch = byte as char;
                    if ch == ' ' {
                        '0'
                    } else {
                        ch
                    }
                })
                .collect::<String>()
                .parse::<i32>()
                .unwrap()
        };
    
        let mut scratch_cards_won = HashMap::new();
    
        let mut n_lines = 0;
        for line in fs::read_to_string("./src/d04/input.txt").unwrap().lines() {
            let card_number = parse_number(line, 5, 3) as usize;
    
            let mut i = line.bytes().position(|ch| ch as char == ':').unwrap() + 2;
            let mut winning_numbers = HashSet::new();
    
            while line.as_bytes()[i] as char != '|' {
                let winning_number = parse_number(line, i, 2);
    
                winning_numbers.insert(winning_number);
    
                i += 3;
            }
    
            i += 2;
    
            let mut n_matches = 0;
    
            while i < line.len() {
                let candidate_number = parse_number(line, i, 2);
    
                if winning_numbers.contains(&candidate_number) {
                    n_matches += 1;
                }
                
                i += 3;
            }
    
            scratch_cards_won.insert(card_number - 1, n_matches);
            n_lines += 1;
        }
    
        let mut cards_won_per_card = vec![0; n_lines];
    
        for i in (0..=n_lines - 2).rev() {
            let mut n_cards_won = 0;
            for j in i + 1..=i + scratch_cards_won.get(&i).unwrap().min(&(n_lines - 1)) {
                n_cards_won += 1 + cards_won_per_card[j];
            }
            cards_won_per_card[i] = n_cards_won;
        }
    
        cards_won_per_card.iter().sum::<i32>() + n_lines as i32
    }
}