use crate::utils::grid::{Grid, Vec2};
use std::collections::{HashMap, HashSet};

pub fn exec(input: &str) -> (usize, usize) {
    let grid = Grid::parse(input);

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

    let mut cache = HashMap::new();

    let part_2 = count_paths(start, 0, height - 1, &grid, &mut cache);

    (splits.len(), part_2)
}

fn count_paths(
    start: Vec2,
    prev: usize,
    max_distance: usize,
    grid: &Grid,
    cache: &mut HashMap<Vec2, usize>,
) -> usize {
    if cache.contains_key(&start) {
        return *cache.get(&start).unwrap();
    }

    if start.y == max_distance {
        return 1;
    }

    let mut count = prev;

    match grid.get(Vec2 {
        x: start.x,
        y: start.y + 1,
    }) {
        Some('.') => {
            count += count_paths(
                Vec2 {
                    x: start.x,
                    y: start.y + 1,
                },
                prev,
                max_distance,
                grid,
                cache,
            );
        }
        Some('^') => {
            count += count_paths(
                Vec2 {
                    x: start.x - 1,
                    y: start.y + 1,
                },
                prev,
                max_distance,
                grid,
                cache,
            );
            count += count_paths(
                Vec2 {
                    x: start.x + 1,
                    y: start.y + 1,
                },
                prev,
                max_distance,
                grid,
                cache,
            );
        }
        _ => panic!("oops"),
    };

    cache.insert(start, count);

    count
}

pub fn _exec_adam(input: &str) -> (usize, usize) {
    let grid = Grid::parse(input);
    let start = grid.find_one('S').unwrap();

    let mut part_1 = 0;
    let grid_size = grid.get_size();
    let mut tachyon_counts = vec![0; grid_size.x];

    tachyon_counts[start.x] = 1;

    for y in 0..grid_size.y {
        for x in 0..grid_size.x {
            let tachyon_count = tachyon_counts[x];
            if tachyon_count == 0 {
                continue;
            }

            if let Some('^') = grid.get(Vec2 { x, y }) {
                part_1 += 1;
                tachyon_counts[x - 1] += tachyon_count;
                tachyon_counts[x] = 0;
                tachyon_counts[x + 1] += tachyon_count;
            };
        }
    }

    (part_1, tachyon_counts.iter().sum())
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
        assert_eq!(result.1, 40);
    }
}
