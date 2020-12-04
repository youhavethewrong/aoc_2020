extern crate aoc_2020;

use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error};
use std::result::Result;

use aoc_2020::day2::{parse_password_string, Password};

fn main() -> Result<(), Error> {
    let input = "input/day2_a.txt";
    let passwords = convert_file_contents_to_passwords(&input)?;
    let result: Vec<bool> = passwords
        .iter()
        .map(|p| p.is_valid_for_toboggan_company())
        .filter(|p| *p)
        .collect();
    println!("{} of {}", result.len(), passwords.len());
    Ok(())
}

#[allow(dead_code)]
fn convert_file_contents_to_numbers(input: &str) -> Result<Vec<u32>, Error> {
    let file = File::open(input)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let lines = contents.split("\n");
    let mut numbers: Vec<u32> = vec![];
    for line in lines {
        if line.len() > 0 {
            let number: u32 = line.parse().unwrap();
            numbers.push(number);
        }
    }
    Ok(numbers)
}

fn convert_file_contents_to_passwords(input: &str) -> Result<Vec<Password>, Error> {
    let file = File::open(input)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let lines = contents.split("\n");
    let mut passwords: Vec<Password> = vec![];
    for line in lines {
        if line.len() > 0 {
            let password: Password = parse_password_string(line);
            passwords.push(password);
        }
    }
    Ok(passwords)
}
