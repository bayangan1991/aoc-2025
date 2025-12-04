#[derive(PartialEq)]
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


pub fn exec(input: &str) -> (usize, usize) {
    let mut part_1 = 0;
    let grid = input.lines().map(parse_line).collect::<Vec<_>>();

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

            part_1 += if nw + n + ne + e + se + s + sw + w < 4 { 1 } else { 0 }
        }
    }

    (part_1, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_input;

    #[test]
    fn test_sample() {
        let sample = read_input("04_sample");
        let result = exec(&sample);
        assert_eq!(result.0, 13)
    }
}