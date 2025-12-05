pub struct Grid {
    pub data: Vec<Vec<char>>,
}

pub fn parse_grid(input: &str) -> Grid {
    Grid {
        data: input.lines().map(|line| line.chars().collect()).collect(),
    }
}

impl Grid {
    pub fn get_size(&self) -> (usize, usize) {
        (
            self.data.len(),
            self.data.first().map(|line| line.len()).unwrap_or(0),
        )
    }

    pub fn count_adjacent(&self, x: usize, y: usize, find: char, include_diagonals: bool) -> usize {
        let mut count = 0;
        let grid_size = self.get_size().0 - 1;

        let n = if y == 0 {
            0
        } else if self.data[y - 1][x] == find {
            1
        } else {
            0
        };
        let e = if x == grid_size {
            0
        } else if self.data[y][x + 1] == find {
            1
        } else {
            0
        };
        let s = if y == grid_size {
            0
        } else if self.data[y + 1][x] == find {
            1
        } else {
            0
        };
        let w = if x == 0 {
            0
        } else if self.data[y][x - 1] == find {
            1
        } else {
            0
        };

        count += n + e + s + w;

        if include_diagonals {
            let nw = if y == 0 || x == 0 {
                0
            } else if self.data[y - 1][x - 1] == find {
                1
            } else {
                0
            };
            let ne = if y == 0 || x == grid_size {
                0
            } else if self.data[y - 1][x + 1] == find {
                1
            } else {
                0
            };
            let se = if y == grid_size || x == grid_size {
                0
            } else if self.data[y + 1][x + 1] == find {
                1
            } else {
                0
            };
            let sw = if y == grid_size || x == 0 {
                0
            } else if self.data[y + 1][x - 1] == find {
                1
            } else {
                0
            };

            count += nw + ne + se + sw
        }

        count
    }
}
