use crate::utils::read_input;

mod days;
mod utils;

fn main() {
    let day_01 = days::day_01::exec(&read_input("01"));
    println!("Day 01: {:?}", day_01);
}
