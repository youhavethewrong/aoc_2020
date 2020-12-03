pub fn find_two(input: Vec<u32>, target: u32) -> u32 {
    for x in &input {
        for y in &input {
            if x + y == target {
                return x * y;
            }
        }
    }
    0
}

pub fn find_three(input: Vec<u32>, target: u32) -> u32 {
    for x in &input {
        for y in &input {
            for z in &input {
                if x + y + z == target {
                    return x * y * z;
                }
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_2020_two() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        let expected = 514579;
        assert_eq!(expected, find_two(input, 2020));
    }

    #[test]
    fn should_find_2020_three() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        let expected = 241861950;
        assert_eq!(expected, find_three(input, 2020));
    }
}
