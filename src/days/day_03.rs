use rayon::prelude::*;

fn get_joltage(bank_str: &str, length: usize, prev: usize) -> usize {
    let bank = bank_str.chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<_>>();

    let search_space = &bank[0..bank_str.len() - length + 1];
    let (index, max_digit) = search_space.iter().enumerate().reduce(|acc, next| if next.1 > acc.1 { next } else { acc }).unwrap();

    if length == 1 { prev + *max_digit } else {
        let (_, new_str) = bank_str.split_at(index + 1);
        get_joltage(new_str, length - 1, (prev + *max_digit) * 10)
    }
}

pub fn exec(input: &str) -> (usize, usize) {
    let part_1 = input.par_lines().map(|line| get_joltage(line, 2, 0)).sum::<usize>();
    let part_2 = input.par_lines().map(|line| get_joltage(line, 12, 0)).sum::<usize>();

    (part_1, part_2)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_input;
    #[test]
    fn test_simple_joltage() {
        assert_eq!(get_joltage("987654321111111", 12, 0), 987654321111);
        assert_eq!(get_joltage("2215452689925244273244333436189317446384838478525478824435233342352236255624326767355438753493222423", 2, 0), 99);
        assert_eq!(get_joltage("11", 2, 0), 11);
        assert_eq!(get_joltage("121", 2, 0), 21);
        assert_eq!(get_joltage("1219", 2, 0), 29);
        assert_eq!(get_joltage("192", 2, 0), 92);
        assert_eq!(get_joltage("11989", 2, 0), 99);
        assert_eq!(get_joltage("1122", 2, 0), 22);
        assert_eq!(get_joltage("09876543210", 2, 0), 98);
        assert_eq!(get_joltage("0000", 2, 0), 0);
        assert_eq!(get_joltage("9000", 2, 0), 90);
        assert_eq!(get_joltage("0009", 2, 0), 9);
        assert_eq!(get_joltage("987654321111111", 2, 0), 98);
        assert_eq!(get_joltage("811111111111119", 2, 0), 89);
        assert_eq!(get_joltage("234234234234278", 2, 0), 78);
        assert_eq!(get_joltage("818181911112111", 2, 0), 92);
        assert_eq!(get_joltage("811111111111119", 12, 0), 811111111119);
        assert_eq!(get_joltage("234234234234278", 12, 0), 434234234278);
        assert_eq!(get_joltage("818181911112111", 12, 0), 888911112111);
    }

    #[test]
    fn test_sample() {
        let input = read_input("03_sample");
        let result = exec(&input);

        assert_eq!(result.0, 357)
    }
}