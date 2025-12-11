use rayon::prelude::*;
use std::collections::HashMap;

fn parse(input: &str) -> HashMap<String, Vec<String>> {
    input
        .par_lines()
        .map(|line| {
            let (id, links) = line.split_once(": ").unwrap();
            let id = id.to_string();

            let links = links.split(' ').map(|l| l.to_string()).collect::<Vec<_>>();
            (id, links)
        })
        .fold(
            || HashMap::with_capacity(input.lines().count()),
            |mut map, (id, links)| {
                map.insert(id.to_string(), links);
                map
            },
        )
        .reduce_with(|mut map, new| {
            for (id, links) in new {
                map.insert(id, links);
            }
            map
        })
        .unwrap()
}

pub fn exec(input: &str) -> (usize, usize) {
    let map = parse(input);

    let part_1 = cached_count_paths("you", "out", &map, true, true);
    let part_2 = cached_count_paths("svr", "out", &map, false, false);

    (part_1, part_2)
}

pub fn exec_evan(input: &str) -> (usize, usize) {
    let map = parse(input);
    let part_1 = cached_count_paths("you", "out", &map, true, true);

    let srv_fft = cached_count_paths("svr", "fft", &map, true, true);
    let fft_dac = cached_count_paths("fft", "dac", &map, true, true);
    let dac_out = cached_count_paths("dac", "out", &map, true, true);

    let srv_dac = cached_count_paths("svr", "dac", &map, true, true);
    let dac_fft = cached_count_paths("dac", "fft", &map, true, true);
    let fft_out = cached_count_paths("fft", "out", &map, true, true);

    let part_2 = (srv_fft * fft_dac * dac_out) + (srv_dac * dac_fft * fft_out);

    (part_1, part_2)
}

fn cached_count_paths(
    node: &str,
    dest: &str,
    map: &HashMap<String, Vec<String>>,
    visited_fft: bool,
    visited_dac: bool,
) -> usize {
    let mut cache = HashMap::with_capacity(1500);
    count_paths(node, dest, map, visited_fft, visited_dac, &mut cache)
}

fn count_paths(
    node: &str,
    dest: &str,
    map: &HashMap<String, Vec<String>>,
    visited_fft: bool,
    visited_dac: bool,
    cache: &mut HashMap<(String, bool, bool), usize>,
) -> usize {
    let key = (node.to_string(), visited_fft, visited_dac);

    if node == dest {
        let result = if visited_fft && visited_dac { 1 } else { 0 };
        return result;
    }

    let visited_fft = visited_fft || node == "fft";
    let visited_dac = visited_dac || node == "dac";

    if cache.contains_key(&key) {
        return cache[&key];
    }

    if let Some(links) = map.get(node) {
        let result = links
            .iter()
            .map(|link| count_paths(link, dest, map, visited_fft, visited_dac, cache))
            .sum();
        cache.insert(key, result);
        result
    } else {
        cache.insert(key, 0);
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::file::read_input;
    use std::collections::HashMap;

    #[test]
    fn test_sample() {
        let input = read_input("11_sample");
        let result = exec(&input);
        assert_eq!(result.0, 5);
    }

    #[test]
    fn test_sample_2() {
        let input = read_input("11_sample_2");
        let result = exec(&input);
        assert_eq!(result.1, 2);
    }

    #[test]
    fn test_parse() {
        let input = "aaa: you hhh\nhhh: out";
        let map = parse(input);
        assert_eq!(
            map,
            HashMap::from([
                (
                    "aaa".to_string(),
                    vec!["you".to_string(), "hhh".to_string()]
                ),
                ("hhh".to_string(), vec!["out".to_string()])
            ])
        )
    }
}
