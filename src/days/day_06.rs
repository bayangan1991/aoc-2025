pub fn exec(input: &str) -> (usize, usize) {
    let puzzles = input
        .lines()
        .map(|line| line.split_ascii_whitespace().collect())
        .collect::<Vec<Vec<_>>>();

    let part_1_size = puzzles[0].len();

    let part_1 = (0..part_1_size)
        .map(|i| {
            let mut parts = puzzles.iter().map(|part| part[i]).collect::<Vec<_>>();
            let operator = parts.pop().unwrap();

            let parts = parts
                .iter()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            match operator {
                "*" => parts.iter().product(),
                "+" => parts.iter().sum::<usize>(),
                _ => panic!("Unknown operator: {}", operator),
            }
        })
        .sum();

    let lines = input.lines().collect::<Vec<_>>();

    let height = lines.len();
    let width = lines.iter().map(|l| l.len()).max().unwrap();

    let mut part_2 = 0;
    let mut nums = Vec::new();
    let mut num_string: String;

    for i in (0..width).rev() {
        num_string = String::from_utf8(
            lines[0..height - 1]
                .iter()
                .copied()
                .map(|c| *c.as_bytes().get(i).unwrap_or(&0x20u8))
                .collect::<Vec<_>>(),
        )
        .unwrap()
        .trim()
        .parse()
        .unwrap();

        if num_string.is_empty() {
            nums.clear();
            continue;
        }

        let num = num_string.parse::<usize>().unwrap();
        nums.push(num);

        let operator = *lines[height - 1].as_bytes().get(i).unwrap_or(&0x20u8) as char;

        match operator {
            '+' => part_2 += nums.iter().sum::<usize>(),
            '*' => part_2 += nums.iter().product::<usize>(),
            _ => {}
        };
    }

    (part_1, part_2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::file::read_input;

    #[test]
    fn test_sample() {
        let input = read_input("06_sample");

        let result = exec(&input);
        assert_eq!(result.0, 4277556);
        assert_eq!(result.1, 3263827);
    }
}
