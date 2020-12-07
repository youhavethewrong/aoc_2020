pub struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl Passport {
    is_valid(&self) -> bool {
        false
    }
}

pub fn parse_passport_data(input: &str) -> Passport {
    Passport {}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_parse_passport_data() {
        let input = r"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm";
        let expected = Passport {
            eye_color = Some("gry"),
            passport_id = Some("860033327"),
            expiration_year = Some("2020"),
            hair_color = Some("#fffffd"),
            birth_year = Some("1937"),
            issue_year = Some("2017"),
            country_id = Some("147"),
            height = Some("183cm"),
        };
        let actual = parse_passport_data(input);
        assert_eq!(expected, actual);
    }
}
