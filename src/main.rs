use std::env;
use std::fs;
use std::fmt::Display;

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;

trait Solver<T> {
    fn run_01() -> T;
    fn run_02() -> T;
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let args = env::args().collect::<Vec<_>>();

    if let Some(day_number) = args.get(1) {
        if day_number == "uncarriage" {
            if let Some(file_path) = args.get(2) {
                let file_contents = fs::read_to_string(file_path).unwrap();
                fs::write(file_path, file_contents.replace("\r\n", "\n")).unwrap();
            } else {
                println!("Please supply a file path!");
            }
            return;
        }

        if let Ok(n_day) = day_number.parse::<i32>() {
            println!("Day {}:", n_day);

            match n_day {
                1 => execute_day::<d01::Day, i32>(),
                2 => execute_day::<d02::Day, i32>(),
                3 => execute_day::<d03::Day, i32>(),
                4 => execute_day::<d04::Day, i32>(),
                5 => execute_day::<d05::Day, i64>(),

                _ => println!("Day number is out of bounds!"),
            };
        } else {
            println!("Day number is not a number!");
        }
    } else {
        println!("Please supply a day number as an argument!");
    }
}

fn execute_day<S: Solver<U>, U: Display>() {
    println!("{}", S::run_01());
    println!("{}", S::run_02());
}