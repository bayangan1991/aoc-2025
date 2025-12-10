use crate::utils::grid::Vec2;
use crate::utils::itertools::pairwise;

type Point = Vec2<isize>;
type Edge = (Point, Point);

pub fn exec(input: &str) -> (usize, usize) {
    let points = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            Vec2 {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect::<Vec<Point>>();

    let combinations = points
        .iter()
        .enumerate()
        .flat_map(|(i, a)| points.iter().skip(i + 1).map(|b| (*a, *b)))
        .collect::<Vec<_>>();

    let sizes = combinations
        .iter()
        .map(|(a, b)| find_area(a, b))
        .collect::<Vec<_>>();

    let part_1 = sizes.iter().max().unwrap();

    let edges = pairwise(&points)
        .filter_map(|(a, b)| match b {
            Some(b) => Some((*a, *b)),
            None => None,
        })
        .chain([(points[points.len() - 1], points[0])])
        .collect::<Vec<Edge>>();

    let mut part_2 = 0;
    for (a, b) in combinations.iter() {
        let rect = [*a, Point { x: b.x, y: a.y }, *b, Point { x: a.x, y: b.y }];
        let inside = rect
            .iter()
            .map(|p| is_point_in_polygon(p, &points))
            .all(|b| b);

        if !inside {
            continue;
        }
        let rect_edges = [
            (rect[0], rect[1]),
            (rect[1], rect[2]),
            (rect[2], rect[3]),
            (rect[3], rect[0]),
        ];
        let mut intersects = false;
        for edge in edges.iter() {
            for rect_edge in rect_edges.iter() {
                if edges_intersect(edge, rect_edge) {
                    intersects = true;
                    break;
                }
            }
        }
        if !intersects {
            part_2 = part_2.max(find_area(a, b));
        }
    }

    (*part_1, part_2)
}

fn ccw(a: &Point, b: &Point, c: &Point) -> isize {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

fn edges_intersect(edge_1: &Edge, edge_2: &Edge) -> bool {
    let (p1, p2) = edge_1;
    let (p3, p4) = edge_2;

    let d1 = ccw(&p1, &p2, &p3);
    let d2 = ccw(&p1, &p2, &p4);
    let d3 = ccw(&p3, &p4, &p1);
    let d4 = ccw(&p3, &p4, &p2);

    ((d1 > 0 && d2 < 0) || (d1 < 0 && d2 > 0)) && ((d3 > 0 && d4 < 0) || (d3 < 0 && d4 > 0))
}

fn is_point_in_polygon(point: &Point, poly: &Vec<Point>) -> bool {
    let size = poly.len();
    let mut inside = false;

    let mut p1 = poly[size - 1];

    for i in 0..size {
        let p2 = poly[i];

        let cross_product = ccw(&p1, &p2, point);

        let min = Point {
            x: p1.x.min(p2.x),
            y: p1.y.min(p2.y),
        };
        let max = Point {
            x: p1.x.max(p2.x),
            y: p1.y.max(p2.y),
        };
        if cross_product == 0
            && (min.x <= point.x)
            && (point.x <= max.x)
            && (min.y <= point.y)
            && (point.y <= max.y)
        {
            return true;
        }

        if point.y > min.y && point.y <= max.y && point.x <= max.x {
            let mut x_ints = p1.x;
            if p1.y != p2.y {
                x_ints = (point.y - p1.y) * (p2.x - p1.x) / (p2.y - p1.y) + p1.x;
            }
            if p1.x == p2.x || point.x <= x_ints {
                inside = !inside;
            }
        }

        p1 = p2;
    }

    inside
}

fn find_area(a: &Point, b: &Point) -> usize {
    (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1)
}

pub fn exec_jarek(input: &str) -> (usize, usize) {
    let points = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            Vec2 {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect::<Vec<Point>>();

    let mut rectangles = points
        .iter()
        .enumerate()
        .flat_map(|(i, a)| points.iter().skip(i + 1).map(|b| (*a, *b)))
        .map(|(a, b)| (find_area(&a, &b), (a, b)))
        .collect::<Vec<_>>();

    rectangles.sort_by_key(|(area, _)| *area);
    rectangles.reverse();

    let part_1 = rectangles[0].0;

    let mut a = points[points.len() - 1];
    let mut v_lines = Vec::with_capacity(rectangles.len() / 2);
    let mut h_lines = Vec::with_capacity(rectangles.len() / 2);
    for b in points.iter() {
        if b.y == a.y {
            h_lines.push(HLine {
                y: a.y,
                x1: a.x.min(b.x),
                x2: a.x.max(b.x),
            });
        } else {
            v_lines.push(VLine {
                x: a.x,
                y1: a.y.min(b.y),
                y2: a.y.max(b.y),
            });
        }
        a = *b;
    }

    v_lines.sort_by_key(|line| line.x);
    h_lines.sort_by_key(|line| line.y);

    let mut part_2 = 0;
    for (area, (a, b)) in rectangles.iter() {
        if rect_intersects_polygon(
            &Point {
                x: a.x.min(b.x),
                y: a.y.min(b.y),
            },
            &Point {
                x: a.x.max(b.x),
                y: a.y.max(b.y),
            },
            &h_lines,
            &v_lines,
        ) {
            continue;
        }
        if point_inside_polygon(
            &Point {
                x: a.x.min(b.x) + 1,
                y: a.y.min(b.y) + 1,
            },
            &v_lines,
        ) {
            continue;
        }

        part_2 = *area;
        break;
    }

    (part_1, part_2)
}

fn rect_intersects_polygon(a: &Point, b: &Point, h_lines: &[HLine], v_lines: &[VLine]) -> bool {
    for h_line in [
        HLine {
            x1: a.x,
            x2: b.x,
            y: a.y,
        },
        HLine {
            x1: a.x,
            x2: b.x,
            y: b.y,
        },
    ] {
        for v_line in v_lines {
            if v_line.x <= h_line.x1 {
                continue;
            }
            if v_line.x >= h_line.x2 {
                break;
            }
            if (h_line.y == v_line.y1 || h_line.y == v_line.y2)
                && a.y <= v_line.y1
                && v_line.y1 <= b.y
                && a.y <= v_line.y2
                && v_line.y2 <= b.y
            {
                return true;
            }
            if v_line.y1 < h_line.y && h_line.y < v_line.y2 {
                return true;
            }
        }
        for v_line in [
            VLine {
                x: a.x,
                y1: a.y,
                y2: b.y,
            },
            VLine {
                x: b.x,
                y1: a.y,
                y2: b.y,
            },
        ] {
            for h_line in h_lines {
                if h_line.y <= v_line.y1 {
                    continue;
                }
                if h_line.y >= v_line.y2 {
                    break;
                }
                if (v_line.x == h_line.x1 || v_line.x == h_line.x2)
                    && a.x <= h_line.x1
                    && h_line.x1 <= b.x
                    && a.x <= h_line.x2
                    && h_line.x2 <= b.x
                {
                    return true;
                }
                if h_line.x1 < v_line.x && v_line.x < h_line.x2 {
                    return true;
                }
            }
        }
    }

    false
}

fn point_inside_polygon(point: &Point, v_lines: &[VLine]) -> bool {
    let mut is_outside = true;
    for v_line in v_lines.iter() {
        if v_line.x >= point.x {
            return is_outside;
        }
        if (v_line.y1 as f64) < (point.y as f64 + 0.1) {
            is_outside = !is_outside;
        }
    }

    false
}

struct VLine {
    y1: isize,
    y2: isize,
    x: isize,
}

struct HLine {
    x1: isize,
    x2: isize,
    y: isize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::file::read_input;

    #[test]
    fn test_sample() {
        let input = read_input("09_sample");
        let result = exec_jarek(&input);
        assert_eq!(result.0, 50);
        assert_eq!(result.1, 24);
    }
}
