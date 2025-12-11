use rayon::prelude::*;
use std::ops::{Shl, ShlAssign};

struct Machine {
    desired: usize,
    current: usize,
    presses: usize,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>,
}

pub fn exec(input: &str) -> (usize, usize) {
    let machines = input.par_lines().map(parse).collect::<Vec<_>>();

    (0, 0)
}

fn solve_machine(machine: &Machine) -> usize {
    for button in machine.buttons.iter() {
        let mut new_current = machine.current;
        for flip in button.iter() {
            new_current ^= (1 << *flip);
        }
        if new_current == machine.current {
            return machine.presses + 1;
        }
    }

    panic!("Unsolvable");
}

fn parse(line: &str) -> Machine {
    let (desired_str, rest) = line.split_once("] ").unwrap();
    let mut desired = 0usize;
    desired_str.chars().skip(1).for_each(|c| {
        desired = desired << 1 ^ if c == '#' { 1 } else { 0 };
    });
    let (button_strs, rest) = rest.split_once(" {").unwrap();
    let buttons = button_strs
        .split(' ')
        .map(|c| {
            c[1..c.len() - 1]
                .split(',')
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let joltages = rest[0..rest.len() - 1]
        .split(',')
        .map(|c| c.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    Machine {
        desired,
        current: 0,
        presses: 0,
        buttons,
        joltages,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::file::read_input;

    #[test]
    fn test_parse() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let machine = parse(input);

        assert_eq!(machine.desired, 6);
        assert_eq!(machine.current, 0);
        assert_eq!(
            machine.buttons,
            vec![
                vec![3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![0, 2],
                vec![0, 1]
            ]
        );
        assert_eq!(machine.joltages, vec![3, 5, 4, 7]);
    }

    #[test]
    fn test_sample() {
        let input = read_input("10_sample");
        let result = exec(&input);
        assert_eq!(result.1, 7)
    }
}
