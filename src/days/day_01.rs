pub fn exec(input: &str) -> (i32, i32) {
    let mut pos = 50;
    let mut previous_pos = 50;
    let mut count_zero = 0;
    let mut count_passed_zero = 0;

    for line in input.lines() {
        let (l, r) = line.split_at(1);
        let value = r.parse::<i32>().unwrap();
        count_passed_zero += if value > 100 { value / 100 } else { 0 };
        match l {
            "L" => {
                pos = (pos - value + 100000) % 100;
                count_passed_zero += if (pos >= previous_pos && previous_pos != 0) || pos == 0 { 1 } else { 0 };
            }
            "R" => {
                pos = (pos + value) % 100;
                count_passed_zero += if pos <= previous_pos { 1 } else { 0 };
            }
            _ => {}
        }
        previous_pos = pos;
        count_zero += if pos == 0 { 1 } else { 0 };
    }

    (count_zero, count_passed_zero)
}

#[cfg(test)]
mod tests {
    use crate::days::day_01::exec;
    use crate::utils::read_input;

    #[test]
    fn example_1() {
        let input = read_input("01_sample");
        let result = exec(&input);
        assert_eq!(result, (3, 6));
    }

    #[test]
    fn example_2() {
        let input = "L100\nR100";
        let result = exec(input);
        assert_eq!(result, (0, 2));
    }
    #[test]
    fn example_3() {
        let input = "L50\nL1000";
        let result = exec(input);
        assert_eq!(result, (2, 12));
    }
    #[test]
    fn example_4() {
        let input = "R50\nR1101";
        let result = exec(input);
        assert_eq!(result, (1, 12));
    }
}
