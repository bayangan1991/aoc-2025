pub fn exec(input: &str) -> (i32, i32) {
    let mut pos = 50;
    let mut count_zero = 0;

    for line in input.lines() {
        let (l, r) = line.split_at(1);
        match l {
            "L" => pos = (pos + r.parse::<i32>().unwrap()) % 100,
            "R" => pos = (pos - r.parse::<i32>().unwrap()) % 100,
            _ => {}
        }
        count_zero += if pos == 0 { 1 } else { 0 };
    }

    (count_zero, 0)
}

#[cfg(test)]
mod tests {
    use crate::days::day_01::exec;
    use crate::utils::read_input;

    #[test]
    fn example_1() {
        let input = read_input("01_sample");
        let result = exec(&input);
        assert_eq!(result.0, 3);
    }
}
