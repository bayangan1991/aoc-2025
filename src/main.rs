use crate::utils::read_input;

mod days;
mod utils;

fn main() {
    let day_01 = days::day_01::exec(&read_input("01"));
    println!("Day 01: {:?}", day_01);
    let day_02 = days::day_02::exec(&read_input("02"));
    println!("Day 02: {:?}", day_02);
    let day_03 = days::day_03::exec(&read_input("03"));
    println!("Day 03: {:?}", day_03);
}
