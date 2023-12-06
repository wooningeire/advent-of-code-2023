use std::fs;
use std::iter;

use crate::Solver;

pub struct Day;

impl Solver<i32> for Day {
    fn run_01() -> i32 {
        let input_string = fs::read_to_string("./src/d06/input.txt").unwrap();
        let mut input_lines = input_string.lines();

        let mut parse_line = || input_lines
                .nth(0)
                .unwrap()
                .split_ascii_whitespace()
                .skip(1)
                .map(|time| time.parse::<i32>().unwrap());

        let race_times = parse_line();
        let race_records = parse_line();

        let mut cumprod = 1;
        for (time, record) in iter::zip(race_times, race_records) {
            let mut n_ways = 0;
            for seconds_held in 0..time {
                if (time - seconds_held) * seconds_held > record {
                    n_ways += 1;
                }
            }
            cumprod *= n_ways;
        }
        cumprod
    }

    fn run_02() -> i32 {
        let input_string = fs::read_to_string("./src/d06/input.txt").unwrap();
        let mut input_lines = input_string.lines();

        let mut parse_line = || input_lines
                .nth(0)
                .unwrap()
                .replace(" ", "")
                .split(":")
                .nth(1)
                .unwrap()
                .parse::<i64>()
                .unwrap();

        let race_time = parse_line();
        let race_record = parse_line();

        for seconds_held in 0..race_time {
            if (race_time - seconds_held) * seconds_held > race_record {
                return (race_time - seconds_held * 2) as i32 + 1;
            }
        }
        0
    }
}