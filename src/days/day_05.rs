pub fn exec(input: &str) -> (usize, usize) {
    let (ranges, ingredients) = parse(input);

    let part_1 = ingredients.iter().map(|i| {
        for range in &ranges {
            if range.0 <= *i && range.1 >= *i {
                return 1;
            } else if range.0 > *i {
                return 0;
            }
        }
        0
    }).sum();

    let mut part_2 = 0;
    let mut curr_range = None;

    for range in &ranges {
        part_2 += match curr_range {
            None => {
                curr_range = Some((range.0, range.1));
                0
            }
            Some((lower, upper)) => if range.0 > upper {
                let result = upper - lower + 1;
                curr_range = Some(*range);
                result
            } else {
                curr_range = Some((lower, range.1.max(upper)));
                0
            }
        }
    }

    let (l, r) = curr_range.unwrap();
    part_2 += r - l + 1;

    (part_1, part_2)
}

fn parse(input: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    let (range_str, ingredients_str) = input.split_once("\n\n").unwrap();
    let mut ranges = range_str.lines().map(|l| l.split_once('-').map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap())).unwrap()).collect::<Vec<(usize, usize)>>();
    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let ingredients = ingredients_str.lines().map(|c| c.parse::<usize>().unwrap()).collect::<Vec<_>>();

    (ranges, ingredients)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::file::read_input;

    #[test]
    fn test_parse() {
        let input = "3-5\n\n1";
        let result = parse(input);

        assert_eq!(result.0, vec![(3, 5)]);
        assert_eq!(result.1, vec![(1)]);
    }

    #[test]
    fn test_sample() {
        let input = read_input("05_sample");
        let result = exec(&input);
        assert_eq!(result.0, 3);
        assert_eq!(result.1, 14);
    }
}