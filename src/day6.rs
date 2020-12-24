use bit_vec::BitVec;

pub fn parse_groups(input: &str) -> Vec<String> {
    input.split("\n\n").map(|b| b.to_owned()).collect()
}

pub fn sum_group_counts(groups: &Vec<String>) -> usize {
    let mut sum = 0;
    for group in groups {
        let mut yesses = BitVec::from_elem(26, false);
        let individuals: Vec<&str> = group.split('\n').collect();
        for i in individuals {
            for c in i.as_bytes() {
                yesses.set(*c as usize - 97, true);
            }
        }

        let yes_count = yesses.iter().filter(|y| *y).collect::<Vec<bool>>().len();
        sum += yes_count;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_and_sum_groups() {
        let input = r"abc

a
b
c

ab
ac

a
a
a
a

b";
        let groups = parse_groups(&input);
        assert_eq!(5, groups.len());

        let sum = sum_group_counts(&groups);
        assert_eq!(11, sum);
    }
}
