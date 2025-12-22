use crate::utils::grid::{Grid, Vec2};
use image::imageops::{resize, FilterType};
use image::ImageBuffer;

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
    let mut grid = Grid::parse(input);

    let mut index = 0;
    draw_grid(&grid, None, "output/day_04/0000.png");

    loop {
        index += 1;
        let (removed, updated_grid) = remove_rolls(&grid);
        if removed == 0 {
            break;
        };
        if part_1 == 0 {
            part_1 = removed
        };
        part_2 += removed;

        let out_name = format!("output/day_04/{:04}.png", index);
        draw_grid(&updated_grid, Some(&grid), &out_name);
        grid = updated_grid;
    }

    (part_1, part_2)
}

fn draw_grid(current: &Grid, before: Option<&Grid>, name: &str) {
    let size = current.get_size();
    let mut imgbuf = ImageBuffer::new(size.x as u32, size.y as u32);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = match current.get(Vec2 {
            x: x as usize,
            y: y as usize,
        }) {
            Some('@') => image::Rgb([255u8; 3]),
            _ => match before {
                None => image::Rgb([0u8; 3]),
                Some(before) => match before.get(Vec2 {
                    x: x as usize,
                    y: y as usize,
                }) {
                    Some('@') => image::Rgb([255u8, 0u8, 0u8]),
                    _ => image::Rgb([0u8; 3]),
                },
            },
        }
    }

    imgbuf = resize(
        &imgbuf,
        size.x as u32 * 4,
        size.y as u32 * 4,
        FilterType::Nearest,
    );

    imgbuf.save(name).unwrap();
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
