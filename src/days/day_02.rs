use rayon::prelude::*;

fn is_doubled(num: usize) -> bool {
    if num < 10 {
        return false;
    };
    let magnitude = num.ilog10() + 1;
    if !magnitude.is_multiple_of(2) {
        return false;
    };

    let left = num / (10usize).pow(magnitude / 2);
    let right = num - left * (10usize).pow(magnitude / 2);

    right == left
}

fn to_digits(v: usize) -> Vec<usize> {
    let mut v_copy = v;
    let mut buf: Vec<usize> = Vec::with_capacity(10);

    while v_copy > 0 {
        let n = v_copy % 10;
        v_copy /= 10;
        buf.push(n);
    }

    buf
}

fn has_repeats(num: usize) -> bool {
    let digits = to_digits(num);
    let num_digits = digits.len();


    for i in 1..=num_digits / 2 {
        if num_digits.is_multiple_of(i) {
            let mut found = true;
            let chunks = digits.chunks(i);
            let mut first = None;
            for chunk in chunks {
                match first {
                    None => first = Some(chunk),
                    Some(value) => {
                        if value != chunk {
                            found = false;
                            break;
                        }
                    }
                }
            }
            if found {
                return true;
            }
        }
    }


    false
}

pub fn exec(input: &str) -> (usize, usize) {
    let (part_1, part_2) = input.par_split(',').map(|range| {
        let (l, r) = range.split_once('-').unwrap();
        let l = l.parse::<usize>().unwrap();
        let r = r.parse::<usize>().unwrap();
        let mut part_1 = 0;
        let mut part_2 = 0;

        for i in (l..=r) {
            part_1 += if is_doubled(i) { i } else { 0 };
            part_2 += if has_repeats(i) { i } else { 0 };
        }

        (part_1, part_2)
    }).reduce(|| (0, 0), |(part_1, part_2), (a, b)| (part_1 + a, part_2 + b));

    (part_1, part_2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_input;
    #[test]
    fn test_sample() {
        let input = read_input("02_sample");
        let (part_1, part_2) = exec(&input);
        assert_eq!(part_1, 1227775554);
        assert_eq!(part_2, 4174379265);
    }


    #[test]
    fn test_is_doubled() {
        assert!(!is_doubled(123124));
        assert!(!is_doubled(12));

        assert!(is_doubled(22));
        assert!(is_doubled(123123));
        assert!(is_doubled(103103));
    }
}
