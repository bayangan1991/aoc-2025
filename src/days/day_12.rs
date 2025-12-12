use crate::utils::grid::Vec2;

pub fn exec(input: &str) -> (usize, usize) {
    let trees = input.lines().skip(30).map(parse_tree).collect::<Vec<_>>();

    let mut fittable_areas = 0;

    for (size, presents) in trees {
        let available_area = size.x * size.y;
        let min_area_required = presents.iter().map(|x| x * 7).sum::<usize>();
        let max_area_required = presents.iter().map(|x| x * 9).sum::<usize>();

        if available_area < min_area_required {
            continue;
        }
        if available_area >= max_area_required {
            fittable_areas += 1;
            continue;
        }
    }

    (fittable_areas, 0)
}

fn parse_tree(input: &str) -> (Vec2, [usize; 6]) {
    let (left, right) = input.split_once(": ").unwrap();
    let (x, y) = left.split_once('x').unwrap();

    let presents = right
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<usize>>();
    (
        Vec2 {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        },
        presents[0..6].try_into().unwrap(),
    )
}
