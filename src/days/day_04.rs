use crate::utils::grid::{parse_grid, Grid, Vec2};

fn remove_rolls(grid: &Grid) -> (usize, Grid) {
    let mut removed = 0;
    let mut updated_grid = grid.data.clone();

    for (y, line) in grid.data.iter().enumerate() {
        for (x, slot) in line.iter().enumerate() {
            if slot != &'@' {
                continue;
            }

            let count = grid.count_adjacent(Vec2 { x, y }, '@', true);

            removed += if count < 4 {
                updated_grid[y][x] = '.';
                1
            } else {
                0
            }
        }
    }

    (removed, Grid { data: updated_grid })
}

pub fn exec(input: &str) -> (usize, usize) {
    let mut part_1 = 0;
    let mut part_2 = 0;
    let mut grid = parse_grid(input);

    loop {
        let (removed, updated_grid) = remove_rolls(&grid);
        if removed == 0 {
            break;
        };
        if part_1 == 0 {
            part_1 = removed
        };
        part_2 += removed;
        grid = updated_grid;
    }

    (part_1, part_2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::file::read_input;

    #[test]
    fn test_sample() {
        let sample = read_input("04_sample");
        let result = exec(&sample);
        assert_eq!(result.0, 13);
        assert_eq!(result.1, 43);
    }
}
