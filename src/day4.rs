#[derive(Debug, PartialEq)]
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
    pub fn new() -> Passport {
        Passport {
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            hair_color: None,
            eye_color: None,
            passport_id: None,
            country_id: None,
        }
    }
    pub fn is_valid_basic(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }

    pub fn is_valid_extended(&self) -> bool {
        let mut birth_year_valid = false;
        let mut issue_year_valid = false;
        let mut expiration_year_valid = false;
        let mut height_valid = false;
        let mut hair_color_valid = false;
        let mut eye_color_valid = false;
        let mut passport_id_valid = false;

        if self.birth_year.is_some() {
            let by = self.birth_year.as_ref().unwrap();
            birth_year_valid = match by.parse::<i64>() {
                Ok(x) => x < 2003 && x > 1919,
                _ => false,
            }
        }

        if self.issue_year.is_some() {
            let iy = self.issue_year.as_ref().unwrap();
            issue_year_valid = match iy.parse::<i64>() {
                Ok(x) => x < 2021 && x > 2009,
                _ => false,
            }
        }

        if self.expiration_year.is_some() {
            let ey = self.expiration_year.as_ref().unwrap();
            expiration_year_valid = match ey.parse::<i64>() {
                Ok(x) => x < 2031 && x > 2019,
                _ => false,
            }
        }

        if self.height.is_some() {
            let h = self.height.as_ref().unwrap();
            let numbers: String = h.matches(char::is_numeric).collect();
            height_valid = if h.ends_with("cm") {
                match numbers.parse::<i64>() {
                    Ok(x) => x > 149 && x < 194,
                    _ => false,
                }
            } else if h.ends_with("in") {
                match numbers.parse::<i64>() {
                    Ok(x) => x > 58 && x < 77,
                    _ => false,
                }
            } else {
                false
            }
        }

        if self.hair_color.is_some() {
            let hc = self.hair_color.as_ref().unwrap();
            hair_color_valid = hc.starts_with('#')
                && hc.len() == 7
                && hc
                    .matches(|c: char| c.is_ascii_hexdigit())
                    .collect::<String>()
                    .len()
                    == 6;
        }

        if self.eye_color.is_some() {
            let ec = self.eye_color.as_ref().unwrap();
            let amber = "amb";
            let blue = "blu";
            let brown = "brn";
            let grey = "gry";
            let green = "grn";
            let hazel = "hzl";
            let other = "oth";

            eye_color_valid = amber == ec
                || blue == ec
                || brown == ec
                || grey == ec
                || green == ec
                || hazel == ec
                || other == ec;
        }

        if self.passport_id.is_some() {
            let pi = self.passport_id.as_ref().unwrap();
            passport_id_valid = pi.len() == 9
                && pi
                    .matches(|c: char| c.is_numeric())
                    .collect::<String>()
                    .len()
                    == 9;
        }

        birth_year_valid
            && issue_year_valid
            && expiration_year_valid
            && height_valid
            && hair_color_valid
            && eye_color_valid
            && passport_id_valid
    }
}

impl Default for Passport {
    fn default() -> Self {
        Self::new()
    }
}

pub fn convert_batch_file_to_passports(input: &str) -> Vec<Passport> {
    let mut passports: Vec<Passport> = Vec::new();
    let lines: Vec<&str> = input.split('\n').collect();
    let mut current = "".to_string();
    for line in lines {
        if line.is_empty() && !current.is_empty() {
            let passport = parse_passport_data(&current);
            passports.push(passport);
            current = "".to_string();
        } else {
            current.push('\n');
            current.push_str(line);
        }
    }
    passports
}

pub fn parse_passport_data(input: &str) -> Passport {
    let input = input.replace('\n', " ");
    let parts: Vec<&str> = input.split(' ').collect();
    let mut passport = Passport::new();
    for part in parts {
        let field: Vec<&str> = part.split(':').collect();
        if field.len() == 2 {
            let name: &str = field[0];
            let value: &str = field[1];
            match name {
                "byr" => passport.birth_year = Some(value.to_string()),
                "iyr" => passport.issue_year = Some(value.to_string()),
                "eyr" => passport.expiration_year = Some(value.to_string()),
                "hgt" => passport.height = Some(value.to_string()),
                "hcl" => passport.hair_color = Some(value.to_string()),
                "ecl" => passport.eye_color = Some(value.to_string()),
                "pid" => passport.passport_id = Some(value.to_string()),
                "cid" => passport.country_id = Some(value.to_string()),
                _ => println!("invalid field {}", name),
            }
        }
    }
    passport
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_parse_passport_data() {
        let input = r"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm";
        let expected = Passport {
            eye_color: Some("gry".to_string()),
            passport_id: Some("860033327".to_string()),
            expiration_year: Some("2020".to_string()),
            hair_color: Some("#fffffd".to_string()),
            birth_year: Some("1937".to_string()),
            issue_year: Some("2017".to_string()),
            country_id: Some("147".to_string()),
            height: Some("183cm".to_string()),
        };
        let actual = parse_passport_data(input);
        assert_eq!(expected, actual);
        assert_eq!(true, actual.is_valid_basic());
    }

    #[test]
    fn should_validate_extended_passport_data() {
        let input = r"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm";
        let expected = Passport {
            eye_color: Some("gry".to_string()),
            passport_id: Some("860033327".to_string()),
            expiration_year: Some("2020".to_string()),
            hair_color: Some("#fffffd".to_string()),
            birth_year: Some("1937".to_string()),
            issue_year: Some("2017".to_string()),
            country_id: Some("147".to_string()),
            height: Some("183cm".to_string()),
        };
        let actual = parse_passport_data(input);
        assert_eq!(expected, actual);
        assert_eq!(true, actual.is_valid_extended());
    }
}
