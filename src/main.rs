use std::env;

mod d01;
mod d02;
mod d03;
mod d04;

trait Solver {
    fn run_01() -> i32;
    fn run_02() -> i32;
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let args = env::args().collect::<Vec<_>>();

    if let Some(day_number) = args.get(1) {
        if let Ok(n_day) = day_number.parse::<i32>() {
            println!("Day {}:", n_day);

            match n_day {
                1 => execute_day::<d01::Day>(),
                2 => execute_day::<d02::Day>(),
                3 => execute_day::<d03::Day>(),
                4 => execute_day::<d04::Day>(),

                _ => println!("Day number is out of bounds!"),
            };
        } else {
            println!("Day number is not a number!");
        }
    } else {
        println!("Please supply a day number as an argument!");
    }
}

fn execute_day<T: Solver>() {
    println!("{}", T::run_01());
    println!("{}", T::run_02());
}