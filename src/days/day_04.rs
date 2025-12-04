#[derive(PartialEq, Clone, Copy)]
enum Thing {
    Paper,
    Nothing,
}
fn parse_line(line: &str) -> Vec<Thing> {
    line.chars().map(|c| match c {
        '@' => Thing::Paper,
        _ => Thing::Nothing,
    }).collect()
}

fn remove_rolls(grid: &Vec<Vec<Thing>>) -> (usize, Vec<Vec<Thing>>) {
    let mut removed = 0;
    let mut updated_grid = grid.clone();

    let grid_size = grid.len() - 1;

    for (y, line) in grid.iter().enumerate() {
        for (x, slot) in line.iter().enumerate() {
            if slot != &Thing::Paper { continue; }

            let nw = if y == 0 || x == 0 { 0 } else if grid[y - 1][x - 1] == Thing::Paper { 1 } else { 0 };
            let n = if y == 0 { 0 } else if grid[y - 1][x] == Thing::Paper { 1 } else { 0 };
            let ne = if y == 0 || x == grid_size { 0 } else if grid[y - 1][x + 1] == Thing::Paper { 1 } else { 0 };
            let e = if x == grid_size { 0 } else if grid[y][x + 1] == Thing::Paper { 1 } else { 0 };
            let se = if y == grid_size || x == grid_size { 0 } else if grid[y + 1][x + 1] == Thing::Paper { 1 } else { 0 };
            let s = if y == grid_size { 0 } else if grid[y + 1][x] == Thing::Paper { 1 } else { 0 };
            let sw = if y == grid_size || x == 0 { 0 } else if grid[y + 1][x - 1] == Thing::Paper { 1 } else { 0 };
            let w = if x == 0 { 0 } else if grid[y][x - 1] == Thing::Paper { 1 } else { 0 };

            removed += if nw + n + ne + e + se + s + sw + w < 4 {
                updated_grid[y][x] = Thing::Nothing;
                1
            } else { 0 }
        }
    }

    (removed, updated_grid)
}


pub fn exec(input: &str) -> (usize, usize) {
    let mut part_1 = 0;
    let mut part_2 = 0;
    let mut grid = input.lines().map(parse_line).collect::<Vec<_>>();

    loop {
        let (removed, updated_grid) = remove_rolls(&grid);
        if removed == 0 { break; };
        if part_1 == 0 { part_1 = removed };
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