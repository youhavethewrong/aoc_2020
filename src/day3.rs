use bit_vec::BitVec;

pub fn place_trees(input: &str) -> BitVec<u32> {
    let input = input.trim();
    let input = input.replace("\n", "");
    let capacity: usize = input.len();
    let mut trees = BitVec::from_elem(capacity, false);
    for (i, c) in input.chars().enumerate() {
        if c == '#' {
            let location = i;
            trees.set(location as usize, true);
        }
    }
    trees
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_place_trees() {
        let base_map = r"
..##.......
#...#...#..
";
        let mut expected: BitVec<u32> = BitVec::from_elem(22, false);
        expected.set(2, true);
        expected.set(3, true);
        expected.set(11, true);
        expected.set(15, true);
        expected.set(19, true);
        // simulate 10 rightward maps
        let actual = place_trees(&base_map);
        assert_eq!(expected, actual);
    }
}
