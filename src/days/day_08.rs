use crate::utils::grid::Vec3;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

pub fn exec(input: &str, limit: usize) -> (usize, usize) {
    let junction_boxes = parse(input);

    let mut distances = junction_boxes
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            junction_boxes.iter().skip(i + 1).map(|&b| {
                let distance = a.distance(&b);
                (distance, (*a, b))
            })
        })
        .collect::<Vec<_>>();

    distances.sort_by(|a, b| b.0.total_cmp(&a.0));
    distances.reverse();

    let mut jbs_to_circuits = HashMap::with_capacity(junction_boxes.len());
    for (i, jb) in junction_boxes.iter().enumerate() {
        jbs_to_circuits.insert(*jb, i);
    }
    let mut circuits_to_jbs: HashMap<usize, RefCell<HashSet<Vec3>>> =
        HashMap::with_capacity(junction_boxes.len());
    for (jb, circuit) in jbs_to_circuits.iter() {
        circuits_to_jbs.insert(*circuit, RefCell::new(HashSet::from([*jb])));
    }

    for (_, (a, b)) in distances[0..limit].iter() {
        merge_circuits(*a, *b, &mut jbs_to_circuits, &mut circuits_to_jbs);
    }

    let mut circuit_sizes = circuits_to_jbs
        .values()
        .map(|c| c.borrow().len())
        .collect::<Vec<_>>();
    circuit_sizes.sort();
    circuit_sizes.reverse();

    let part_1 = circuit_sizes[0..3].iter().product();
    let part_2;

    let mut start = limit;
    loop {
        let (_, (a, b)) = distances[start];
        merge_circuits(a, b, &mut jbs_to_circuits, &mut circuits_to_jbs);
        if circuits_to_jbs.len() == 1 {
            part_2 = a.x * b.x;
            break;
        }
        start += 1;
    }

    (part_1, part_2)
}

fn merge_circuits(
    a: Vec3,
    b: Vec3,
    jb_to_circuits: &mut HashMap<Vec3, usize>,
    circuits_to_jbs: &mut HashMap<usize, RefCell<HashSet<Vec3>>>,
) {
    let a_id = *jb_to_circuits.get(&a).unwrap();
    let b_id = *jb_to_circuits.get(&b).unwrap();
    if a_id == b_id {
        return;
    }

    let to_merge = circuits_to_jbs.get(&b_id).unwrap().borrow().clone();
    for jb in to_merge.iter() {
        jb_to_circuits.insert(*jb, a_id);
        circuits_to_jbs
            .get_mut(&a_id)
            .unwrap()
            .borrow_mut()
            .insert(*jb);
    }
    circuits_to_jbs.remove(&b_id);
}

fn parse(input: &str) -> Vec<Vec3> {
    input
        .lines()
        .map(|line| {
            let parts = line.split(',').collect::<Vec<_>>();
            Vec3 {
                x: parts[0].parse().unwrap(),
                y: parts[1].parse().unwrap(),
                z: parts[2].parse().unwrap(),
            }
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::file::read_input;
    use crate::utils::grid::Vec3;

    #[test]
    fn test_parse() {
        let input = "1,2,3\n4,5,6";
        let points = parse(input);
        assert_eq!(points[0], Vec3 { x: 1, y: 2, z: 3 });
        assert_eq!(points[1], Vec3 { x: 4, y: 5, z: 6 });
    }

    #[test]
    fn test_sample() {
        let input = read_input("08_sample");
        let result = exec(&input, 10);
        assert_eq!(result.0, 40);
        assert_eq!(result.1, 25272);
    }
}
