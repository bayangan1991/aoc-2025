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
    author: &'static str,
    duration: AutoDuration,
    part_a: usize,
    part_b: usize,
}

fn main() {
    let result = [
        || (1, "Ryan", days::day_01::exec(&read_input("01"))),
        || (2, "Ryan", days::day_02::exec(&read_input("02"))),
        || (3, "Ryan", days::day_03::exec(&read_input("03"))),
        || (4, "Ryan", days::day_04::exec(&read_input("04"))),
        || (5, "Ryan", days::day_05::exec(&read_input("05"))),
        || (6, "Ryan", days::day_06::exec(&read_input("06"))),
        || (7, "Ryan", days::day_07::exec(&read_input("07"))),
        || (8, "Ryan", days::day_08::exec(&read_input("08"), 1000)),
        || (9, "Paul", days::day_09::exec(&read_input("09"))),
        || (9, "Jarek", days::day_09::exec_jarek(&read_input("09"))),
        || (10, "Ryan", days::day_10::exec(&read_input("10"))),
        || (11, "Ryan", days::day_11::exec(&read_input("11"))),
        || (11, "Evan", days::day_11::exec_evan(&read_input("11"))),
        || (12, "Ryan", days::day_12::exec(&read_input("12"))),
    ]
    .par_iter()
    .map(|f| {
        let mut times = Vec::with_capacity(30);
        for _ in 0..30 {
            let start = Instant::now();
            f();
            times.push(Instant::now().duration_since(start));
        }
        let (day, author, result) = f();
        Result {
            day,
            author,
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
