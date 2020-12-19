pub fn find_seat_id(locator: &str) -> u32 {
    let row_locator = &locator[..7];
    let row_string: String = row_locator
        .chars()
        .map(|c| if c == 'F' { "1" } else { "0" })
        .collect();
    let row = 127 - u32::from_str_radix(&row_string, 2).unwrap();

    let col_locator = &locator[7..10];
    let col_string: String = col_locator
        .chars()
        .map(|c| if c == 'R' { "1" } else { "0" })
        .collect();
    let col = u32::from_str_radix(&col_string, 2).unwrap();
    8 * row + col
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_seat_ids() {
        let locator_1 = "FBFBBFFRLR";
        let locator_2 = "BFFFBBFRRR";
        let locator_3 = "FFFBBBFRRR";
        let locator_4 = "BBFFBBFRLL";

        let seat_id1 = find_seat_id(locator_1);
        let seat_id2 = find_seat_id(locator_2);
        let seat_id3 = find_seat_id(locator_3);
        let seat_id4 = find_seat_id(locator_4);
        assert_eq!(357, seat_id1);
        assert_eq!(567, seat_id2);
        assert_eq!(119, seat_id3);
        assert_eq!(820, seat_id4);
    }
}
