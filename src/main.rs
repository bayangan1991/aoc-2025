use rayon::iter::*;
use std::time::Instant;
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
        || days::day_06::exec(&read_input("06")),
        || days::day_07::exec(&read_input("07")),
        || days::day_08::exec(&read_input("08"), 1000),
        || days::day_09::exec(&read_input("09")),
    ]
    .par_iter()
    .enumerate()
    .map(|(index, f)| {
        let mut times = Vec::with_capacity(10);
        for _ in 0..50 {
            let start = Instant::now();
            f();
            times.push(Instant::now().duration_since(start));
        }
        let result = f();
        (index, result, *times.iter().min().unwrap())
    })
    .collect::<Vec<_>>();

    for result in result {
        println!(
            "Day {}: {:?}\t{}\t{}",
            result.0 + 1,
            result.2,
            result.1.0,
            result.1.1,
        );
    }
}
