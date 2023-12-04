use std::env;

mod d01;
mod d02;
mod d03;
mod d04;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    // println!("Day 1:");
    // println!("{}", d01::run_01());
    // println!("{}", d01::run_02());
    
    // println!("Day 2:");
    // println!("{}", d02::run_01());
    // println!("{}", d02::run_02());
    
    // println!("Day 3:");
    // println!("{}", d03::run_01());
    // println!("{}", d03::run_02());
    
    println!("Day 4:");
    println!("{}", d04::run_01());
    println!("{}", d04::run_02());
}
