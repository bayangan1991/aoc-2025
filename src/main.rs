use rayon::iter::*;
use utils::file::read_input;

mod days;
mod utils;

fn main() {
    let result = [
        || days::day_01::exec(&read_input("01")),
        || days::day_02::exec(&read_input("02")),
        || days::day_03::exec(&read_input("03")),
        || days::day_04::exec(&read_input("04")),
        || days::day_05::exec(&read_input("05")),
    ]
    .par_iter()
    .enumerate()
    .map(|(index, f)| (index, f()))
    .collect::<Vec<_>>();

    for result in result {
        println!("Day {}: {}\t{}", result.0 + 1, result.1.0, result.1.1);
    }
}
