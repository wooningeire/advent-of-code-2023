use std::fs;

use crate::Solver;

pub struct Day;

impl Solver for Day {
    fn run_01() -> i32 {
        let input_string = fs::read_to_string("./src/d03/input.txt").unwrap();

        let line_length = input_string.lines().take(1).collect::<Vec<_>>()[0].len() as i32;
        let n_lines = input_string.as_bytes().len() as i32 / (line_length + 1) as i32;

        let mut digit_read = vec![false; input_string.len()];

        let index = |x: i32, y: i32| ((line_length + 1) * y + x) as usize;

        let get_char = |x: i32, y: i32| {
            if x < 0 || y < 0 || x >= line_length || y >= n_lines {
                return '.';
            }
            input_string.as_bytes()[index(x, y)] as char
        };
        let set_read = |digit_read: &mut Vec<bool>, x: i32, y: i32| {
            digit_read[index(x, y)] = true;
        };
        let has_been_read = |digit_read: &Vec<bool>, x: i32, y: i32| {
            if x < 0 || y < 0 || x >= line_length || y >= n_lines {
                return false;
            }
            digit_read[index(x, y)]
        };


        let mut cumsum = 0;
        
        for (j, line) in input_string.lines().enumerate() {
            for (i, ch) in line.chars().enumerate() {
                if ch == '.' || ch.is_digit(10) { continue; }

                for y in j as i32 - 1 .. j as i32 + 2 {
                    for x in i as i32 - 1 .. i as i32 + 2 {
                        if x == i as i32 && y == j as i32 { continue; }
                        if !get_char(x, y).is_digit(10) { continue; }
                        if has_been_read(&digit_read, x, y) { continue; }


                        set_read(&mut digit_read, x, y);

                        let mut number_string = get_char(x, y).to_string();

                        for m in (0..x).rev() {
                            if !get_char(m, y).is_digit(10) { break; }
                            number_string.push(get_char(m, y));

                            set_read(&mut digit_read, m, y);
                        }

                        number_string = String::from_utf8(number_string.bytes().rev().collect::<Vec<_>>()).unwrap();
                        
                        for m in x + 1..line_length + 1 {
                            if !get_char(m, y).is_digit(10) { break; }
                            number_string.push(get_char(m, y));

                            set_read(&mut digit_read, m, y);
                        }

                        cumsum += number_string.parse::<i32>().unwrap();
                    }
                }
            }
        }

        cumsum
    }


    fn run_02() -> i32 {
        let input_string = fs::read_to_string("./src/d03/input.txt").unwrap();

        let line_length = input_string.lines().take(1).collect::<Vec<_>>()[0].len() as i32;
        let n_lines = input_string.as_bytes().len() as i32 / (line_length + 1) as i32;

        let index = |x: i32, y: i32| ((line_length + 1) * y + x) as usize;

        let get_char = |x: i32, y: i32| {
            if x < 0 || y < 0 || x >= line_length || y >= n_lines {
                return '.';
            }
            input_string.as_bytes()[index(x, y)] as char
        };
        

        let mut cumsum = 0;
        
        for (j, line) in input_string.lines().enumerate() {
            for (i, ch) in line.chars().enumerate() {
                if ch != '*' { continue; }

                let mut digit_read = [false; 9];
                let digit_read_index = |x: usize, y: usize| (x + 1 - i + 3 * (y + 1 - j)) as usize;

                let mut n_numbers_read = 0;

                let mut gear_ratio = 1;

                for y in j as i32 - 1 .. j as i32 + 2 {
                    for x in i as i32 - 1 .. i as i32 + 2 {
                        if x == i as i32 && y == j as i32 { continue; }
                        if digit_read[digit_read_index(x as usize, y as usize)] { continue; }
                        if !get_char(x, y).is_digit(10) { continue; }
                        if n_numbers_read >= 2 { continue; }


                        digit_read[digit_read_index(x as usize, y as usize)] = true;

                        let mut number_string = get_char(x, y).to_string();

                        for m in (0..x).rev() {
                            if !get_char(m, y).is_digit(10) { break; }
                            number_string.push(get_char(m, y));

                            if m >= i as i32 - 1 && m <= i as i32 + 1 {
                                digit_read[digit_read_index(m as usize, y as usize)] = true;
                            }
                        }

                        number_string = String::from_utf8(number_string.bytes().rev().collect::<Vec<_>>()).unwrap();
                        
                        for m in x + 1..line_length + 1 {
                            if !get_char(m, y).is_digit(10) { break; }
                            number_string.push(get_char(m, y));

                            if m >= i as i32 - 1 && m <= i as i32 + 1 {
                                digit_read[digit_read_index(m as usize, y as usize)] = true;
                            }
                        }

                        n_numbers_read += 1;
                        gear_ratio *= number_string.parse::<i32>().unwrap();
                    }
                }

                if n_numbers_read == 2 {
                    cumsum += gear_ratio;
                }
            }
        }

        cumsum
    }
}