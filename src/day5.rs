pub fn find_seat_id(locator: &str) -> u32 {
    // take the first 7 characters to find the row
    // use the rest to find the column
    0
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
