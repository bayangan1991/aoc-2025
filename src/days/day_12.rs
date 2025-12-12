pub fn exec(input: &str) -> (usize, usize) {
    let trees = input.lines().skip(30).map(parse_tree).collect::<Vec<_>>();

    (
        trees
            .iter()
            .map(|(area, presents)| {
                let (min_area, max_area) = presents
                    .iter()
                    .map(|x| (x * 7, x * 9))
                    .reduce(|a, b| (a.0 + b.0, a.1 + b.1))
                    .unwrap();

                if *area < min_area {
                    0
                } else if *area >= max_area {
                    1
                } else {
                    panic!("oops")
                }
            })
            .sum::<usize>(),
        0,
    )
}

fn parse_tree(input: &str) -> (usize, [usize; 6]) {
    let (left, right) = input.split_once(": ").unwrap();
    let (x, y) = left.split_once('x').unwrap();

    let presents = right
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<usize>>();
    (
        x.parse::<usize>().unwrap() * y.parse::<usize>().unwrap(),
        presents[0..6].try_into().unwrap(),
    )
}
