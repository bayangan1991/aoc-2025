use std::fs;

pub fn read_input(name: &str) -> String {
    let filepath = format!("inputs/{}", name);
    let err = format!("Unable to find '{filepath}'");
    fs::read_to_string(filepath).expect(&err)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_input() {
        let input = read_input("sample");
        assert_eq!(input, "hello world!");
    }
}
