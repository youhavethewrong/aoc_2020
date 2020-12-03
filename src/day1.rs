pub fn find_2020(input: Vec<u32>) -> u32 {
    for x in &input {
        for y in &input {
            if x + y == 2020 {
                return x * y;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_2020_simple_report() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        let expected = 514579;
        assert_eq!(expected, find_2020(input));
    }
}
