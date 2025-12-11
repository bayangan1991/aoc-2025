use rayon::iter::*;
use std::fmt::Display;
use std::time::{Duration, Instant};
use tabled::settings::Style;
use tabled::{Table, Tabled};
use utils::file::read_input;

mod days;
mod utils;

struct AutoDuration(Duration);
impl Display for AutoDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.0))
    }
}

#[derive(Tabled)]
struct Result {
    day: usize,
    duration: AutoDuration,
    part_a: usize,
    part_b: usize,
}

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
        || days::day_09::exec_jarek(&read_input("09")),
        || days::day_10::exec(&read_input("10")),
    ]
    .par_iter()
    .enumerate()
    .map(|(index, f)| {
        let mut times = Vec::with_capacity(50);
        for _ in 0..50 {
            let start = Instant::now();
            f();
            times.push(Instant::now().duration_since(start));
        }
        let result = f();
        Result {
            day: index + 1,
            duration: AutoDuration(*times.iter().min().unwrap()),
            part_a: result.0,
            part_b: result.1,
        }
    })
    .collect::<Vec<_>>();

    let mut table = Table::new(&result);
    table.with(Style::modern());

    println!("{}", table);
}
