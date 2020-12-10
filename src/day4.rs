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
    pub fn is_valid(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }
}

pub fn convert_batch_file_to_passports(input: &str) -> Vec<Passport> {
    let mut passports: Vec<Passport> = Vec::new();
    let lines: Vec<&str> = input.split('\n').collect();
    let mut current = "".to_string();
    for line in lines {
        if line.len() == 0 && current.len() > 0 {
            let passport = parse_passport_data(&current);
            passports.push(passport);
            current = "".to_string();
        } else {
            current.push_str("\n");
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
    }
}
