pub struct Grid {
    pub data: Vec<Vec<char>>,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Vec2 {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

impl Vec2 {
    pub fn as_tuple(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

pub fn parse_grid(input: &str) -> Grid {
    Grid {
        data: input.lines().map(|line| line.chars().collect()).collect(),
    }
}

impl Grid {
    pub fn get_size(&self) -> Vec2 {
        Vec2 {
            x: self.data.first().map(|line| line.len() + 1).unwrap_or(0),
            y: self.data.len(),
        }
    }

    pub fn count_adjacent(&self, point: Vec2, find: char, include_diagonals: bool) -> usize {
        let mut count = 0;
        let grid_size = self.get_size().y - 1;
        let (x, y) = point.as_tuple();

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

    pub fn find_one(&self, find: char) -> Option<Vec2> {
        let (x_size, y_size) = self.get_size().as_tuple();

        for y in 0..y_size {
            for x in 0..x_size {
                if self.data[y][x] == find {
                    return Some(Vec2 { x, y });
                }
            }
        }

        None
    }

    pub fn get(&self, point: Vec2) -> Option<char> {
        self.data
            .get(point.y)
            .and_then(|row| row.get(point.x).copied())
    }
}
