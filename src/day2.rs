use std::collections::HashMap;

pub struct Password {
    pub min: usize,
    pub max: usize,
    pub required_str: String,
    pub content: String,
    pub freq_analysis: HashMap<String, usize>,
}

impl Password {
    pub fn is_valid(&self) -> bool {
        self.freq_analysis.contains_key(&self.required_str)
            && self.freq_analysis.get(&self.required_str).unwrap() >= &self.min
            && self.freq_analysis.get(&self.required_str).unwrap() <= &self.max
    }
}

pub fn parse_password_string(input: &str) -> Password {
    let parts: Vec<&str> = input.split(" ").collect();
    let count: Vec<&str> = parts[0].split("-").collect();
    let required_str: Vec<&str> = parts[1].split(":").collect();
    let content = parts[2];
    let min: usize = count[0].parse().unwrap();
    let max: usize = count[1].parse().unwrap();
    let mut freq_analysis: HashMap<String, usize> = HashMap::new();
    for c in content.chars() {
        let count = freq_analysis.entry(c.to_string()).or_insert(0);
        *count += 1;
    }
    Password {
        min: min,
        max: max,
        required_str: required_str[0].to_string(),
        content: content.to_string(),
        freq_analysis: freq_analysis,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_build_struct_from_string() {
        let input = "2-9 c: ccccccccc";
        let mut freq_analysis: HashMap<String, usize> = HashMap::new();
        freq_analysis.entry("c".to_string()).or_insert(9);
        let expected = Password {
            min: 2,
            max: 9,
            required_str: "c".to_string(),
            content: "ccccccccc".to_string(),
            freq_analysis,
        };
        let actual = parse_password_string(input);
        assert_eq!(expected.min, actual.min);
        assert_eq!(expected.max, actual.max);
        assert_eq!(expected.required_str, actual.required_str);
        assert_eq!(expected.content, actual.content);
    }

    #[test]
    fn should_validate_passwords() {
        let mut freq_analysis_1: HashMap<String, usize> = HashMap::new();
        freq_analysis_1.entry("c".to_string()).or_insert(9);
        let password_1 = Password {
            min: 2,
            max: 9,
            required_str: "c".to_string(),
            content: "ccccccccc".to_string(),
            freq_analysis: freq_analysis_1,
        };
        assert_eq!(true, password_1.is_valid());

        let mut freq_analysis_2: HashMap<String, usize> = HashMap::new();
        freq_analysis_2.entry("a".to_string()).or_insert(1);
        freq_analysis_2.entry("b".to_string()).or_insert(1);
        freq_analysis_2.entry("c".to_string()).or_insert(1);
        freq_analysis_2.entry("d".to_string()).or_insert(1);
        freq_analysis_2.entry("e".to_string()).or_insert(1);
        let password_2 = Password {
            min: 1,
            max: 3,
            required_str: "a".to_string(),
            content: "abcde".to_string(),
            freq_analysis: freq_analysis_2,
        };
        assert_eq!(true, password_2.is_valid());

        let mut freq_analysis_3: HashMap<String, usize> = HashMap::new();
        freq_analysis_3.entry("c".to_string()).or_insert(1);
        freq_analysis_3.entry("d".to_string()).or_insert(1);
        freq_analysis_3.entry("e".to_string()).or_insert(1);
        freq_analysis_3.entry("f".to_string()).or_insert(1);
        freq_analysis_3.entry("g".to_string()).or_insert(1);
        let password_3 = Password {
            min: 1,
            max: 3,
            required_str: "b".to_string(),
            content: "cdefg".to_string(),
            freq_analysis: freq_analysis_3,
        };
        assert_eq!(false, password_3.is_valid());
    }
}
