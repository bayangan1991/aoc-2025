use rayon::iter::*;
use utils::file::read_input;

mod days;
mod utils;

fn main() {
    let result = [
        || (0, days::day_01::exec(&read_input("01"))),
        || (1, days::day_02::exec(&read_input("02"))),
        || (2, days::day_03::exec(&read_input("03"))),
        || (3, days::day_04::exec(&read_input("04"))),
        || (4, days::day_05::exec(&read_input("05"))),
    ]
    .par_iter()
    .map(|f| f())
    .collect::<Vec<_>>();

    for result in result {
        println!("Day {}: {:?}", result.0 + 1, result.1);
    }
}
