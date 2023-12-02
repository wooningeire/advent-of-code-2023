use std::fs;
use std::collections::HashMap;

pub fn run_01() -> u32 {
    let mut cumsum = 0;
    for line in fs::read_to_string("./src/d01/input.txt").unwrap().lines().map(String::from) {
        let mut first_digit = 0;
        let mut last_digit = 0;

        for ch in line.chars() {
            if let Some(digit) = ch.to_digit(10) {
                first_digit = digit;
                break;
            };
        }

        for ch in line.chars().rev() {
            if let Some(digit) = ch.to_digit(10) {
                last_digit = digit;
                break;
            };
        }

        cumsum += first_digit * 10 + last_digit;
    }

    cumsum
}

pub fn run_02() -> u32 {
    let map = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut cumsum = 0;
    for line in fs::read_to_string("./src/d01/input.txt").unwrap().lines().map(String::from) {
        let mut first_digit = 0;
        let mut last_digit = 0;

        for (word_start, ch) in line.bytes().enumerate() {
            let mut digit_assigned: bool = false;

            if let Some(digit) = (ch as char).to_digit(10) {
                first_digit = digit;
                digit_assigned = true;
            };

            for (&word, &val) in map.iter() {
                let word_end = word_start + word.len();
                if word_end > line.len() { continue; }

                if &line[word_start..word_end] == word {
                    first_digit = val;
                    digit_assigned = true;
                    break;
                }
            }

            if digit_assigned {
                break;
            }
        }

        for (i, ch) in line.bytes().rev().enumerate() {
            let mut digit_assigned: bool = false;

            if let Some(digit) = (ch as char).to_digit(10) {
                last_digit = digit;
                digit_assigned = true;
            };

            for (&word, &val) in map.iter() {
                let word_start = line.len() - i - 1;

                let word_end = word_start + word.len();
                if word_end > line.len() { continue; }

                if &line[word_start..word_end] == word {
                    last_digit = val;
                    digit_assigned = true;
                    break;
                }
            }

            if digit_assigned {
                break;
            }
        }

        cumsum += first_digit * 10 + last_digit;
    }

    cumsum
}