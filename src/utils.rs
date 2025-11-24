use std::fs;
use std::iter;

pub fn read_input(day: &str) -> String {
    let filepath = format!("inputs/{}", day);
    let err = format!("Unable to find '{filepath}'");
    fs::read_to_string(filepath).expect(&err)
}

pub fn pairwise<I>(left: I) -> impl Iterator<Item = (I::Item, Option<I::Item>)>
where
    I: IntoIterator + Clone,
{
    let right = left
        .clone()
        .into_iter()
        .skip(1)
        .map(Some)
        .chain(iter::once(None));
    left.into_iter().zip(right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pairwise() {
        let items = [1, 2, 3];
        let pairwise_items = pairwise(items.iter().cloned()).collect::<Vec<_>>();
        assert_eq!(pairwise_items, vec![(1, Some(2)), (2, Some(3)), (3, None)]);
    }

    #[test]
    fn test_read_input() {
        let input = read_input("sample");
        assert_eq!(input, "hello world!");
    }
}
