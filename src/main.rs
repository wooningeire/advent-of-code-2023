use std::env;

mod d01;
mod d02;
mod d03;
mod d04;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let args = env::args().collect::<Vec<_>>();

    if let Some(day_number) = args.get(1) {
        if let Ok(n_day) = day_number.parse::<i32>() {
            match n_day {
                1 => {
                    println!("Day 1:");
                    println!("{}", d01::run_01());
                    println!("{}", d01::run_02());
                },
                2 => {
                    println!("Day 2:");
                    println!("{}", d02::run_01());
                    println!("{}", d02::run_02());
                },
                3 => {
                    println!("Day 3:");
                    println!("{}", d03::run_01());
                    println!("{}", d03::run_02());
                },
                4 => {
                    println!("Day 4:");
                    println!("{}", d04::run_01());
                    println!("{}", d04::run_02());
                },

                _ => println!("Day number is out of bounds!"),
            };
        } else {
            println!("Day number is not a number!");
        }
    } else {
        println!("Please supply a day number as an argument!");
    }
}
