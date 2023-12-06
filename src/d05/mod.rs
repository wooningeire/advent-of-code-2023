use std::fs;

use crate::Solver;

pub struct Day;

#[derive(Debug)]
struct Range {
    dst_start: i64,
    src_start: i64,
    len: i64,
}

impl Range {
    fn in_range(&self, value: i64) -> bool {
        value >= self.src_start && value < self.src_start + self.len
    }

    fn convert(&self, value: i64) -> i64 {
        value - self.src_start + self.dst_start
    }
}

impl Solver<i64> for Day {
    fn run_01() -> i64 {
        let input_string = fs::read_to_string("./src/d05/input.txt").unwrap();

        let mappings = input_string
                .split("\n\n")
                .skip(1)
                .map(|mapping| mapping
                        .lines()
                        .skip(1)
                        .map(|range_str| {
                            let range_values = range_str
                                    .split(" ")
                                    .map(|value| value.parse::<i64>().unwrap())
                                    .collect::<Vec<_>>();

                            Range {
                                dst_start: range_values[0],
                                src_start: range_values[1],
                                len: range_values[2],
                            }
                        })
                        .collect::<Vec<_>>()
                )
                .collect::<Vec<_>>();

        let seeds = input_string
                .lines()
                .nth(0)
                .unwrap()
                .split(": ")
                .nth(1)
                .unwrap()
                .split(" ")
                .map(|seed_str| seed_str.parse::<i64>().unwrap());

        let locations = seeds.map(|seed| {
            let mut current_value = seed;

            'mapping_loop: for mapping in mappings.iter() {
                for range in mapping {
                    if !range.in_range(current_value) { continue; }

                    current_value = range.convert(current_value);
                    continue 'mapping_loop;
                }
            }
            current_value
        });

        locations.min().unwrap()
    }
    
    fn run_02() -> i64 {
        let input_string = fs::read_to_string("./src/d05/custom.txt").unwrap();

        let mappings = input_string
                .split("\n\n")
                .skip(1)
                .map(|mapping| mapping
                        .lines()
                        .skip(1)
                        .map(|range_str| {
                            let range_values = range_str
                                    .split(" ")
                                    .map(|value| value.parse::<i64>().unwrap())
                                    .collect::<Vec<_>>();

                            Range {
                                dst_start: range_values[0],
                                src_start: range_values[1],
                                len: range_values[2],
                            }
                        })
                        .collect::<Vec<_>>()
                )
                .collect::<Vec<_>>();

        dbg!(mappings);

        let seeds = input_string
                .lines()
                .nth(0)
                .unwrap()
                .split(": ")
                .nth(1)
                .unwrap()
                .split(" ")
                .map(|seed_str| seed_str.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

        let seed_chunks = (0..seeds.len())
                .step_by(2)
                .map(|i| (seeds[i], seeds[i + 1]));

        // locations.min().unwrap()
        0
    }
}