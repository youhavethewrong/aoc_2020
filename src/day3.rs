use bit_vec::BitVec;

pub fn place_trees(input: &str) -> (BitVec<u32>, u32, u32) {
    let input = input.trim();
    let lines: Vec<&str> = input.split("\n").collect();
    let height = lines.len() as u32;
    let width = lines[0].len() as u32;
    let input = input.replace("\n", "");
    let capacity: usize = input.len();
    let mut trees = BitVec::from_elem(capacity, false);
    for (i, c) in input.chars().enumerate() {
        if c == '#' {
            let location = i;
            trees.set(location as usize, true);
        }
    }
    (trees, width, height)
}

pub fn trees_encountered(
    trees: BitVec<u32>,
    width: u32,
    height: u32,
    right: u32,
    down: u32,
) -> u32 {
    // assuming starting at 0 and going 3 right, 1 down
    println!(
        "The map is {} wide by {} high containing {} points.",
        width,
        height,
        trees.len()
    );
    let mut x = 0;
    let mut y = 0;
    let mut counter = 0;
    while y < height {
        let tree_position = x % width + (y * width);
        if trees[tree_position as usize] {
            counter += 1;
        }
        x = (x + right) % width;
        y += down;
    }
    counter
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
        let (trees, width, height) = place_trees(&base_map);
        assert_eq!(11, width);
        assert_eq!(2, height);
        assert_eq!(expected, trees);
    }

    #[test]
    fn should_count_trees_encountered() {
        let base_map = r"
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
";
        let expected = 7;
        let (trees, width, height) = place_trees(base_map);
        let actual = trees_encountered(trees, width, height, 3, 1);
        assert_eq!(expected, actual);
    }
}
