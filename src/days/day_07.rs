use crate::utils::grid::{parse_grid, Vec2};
use std::collections::HashSet;

pub fn exec(input: &str) -> (usize, usize) {
    let grid = parse_grid(input);

    let start = grid.find_one('S').unwrap();

    let mut beams = HashSet::from([start]);
    let mut new_beams = HashSet::from([start]);
    let mut splits = HashSet::new();
    let height = grid.get_size().y;

    'outer: loop {
        new_beams.clear();
        for beam in beams.iter() {
            if beam.y == height - 1 {
                break 'outer;
            }
            match grid.get(Vec2 {
                x: beam.x,
                y: beam.y + 1,
            }) {
                Some('^') => {
                    splits.insert(Vec2 {
                        x: beam.x,
                        y: beam.y + 1,
                    });
                    new_beams.insert(Vec2 {
                        x: beam.x - 1,
                        y: beam.y + 1,
                    });
                    new_beams.insert(Vec2 {
                        x: beam.x + 1,
                        y: beam.y + 1,
                    });
                }
                Some('.') => {
                    new_beams.insert(Vec2 {
                        x: beam.x,
                        y: beam.y + 1,
                    });
                }
                _ => {}
            }
        }

        beams = new_beams.clone();
    }

    (splits.len(), 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::file::read_input;

    #[test]
    fn test_sample() {
        let input = read_input("07_sample");
        let result = exec(&input);
        assert_eq!(result.0, 21);
        assert_eq!(result.1, 0);
    }
}
