extern crate aoc_2020;

use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error};
use std::result::Result;

use aoc_2020::day2::{parse_password_string, Password};
use aoc_2020::day3::{place_trees, trees_encountered};

fn main() -> Result<(), Error> {
    let input = std::env::args().nth(1).expect("\nprovide a filename\n");
    let map = convert_file_to_string(&input)?;
    let (trees, width, height) = place_trees(&map);
    let count = trees_encountered(trees, width, height, 3, 1);
    println!("Encountered {} trees.", count);
    Ok(())
}

fn convert_file_to_string(input: &str) -> Result<String, Error> {
    let file = File::open(input)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
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

#[allow(dead_code)]
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
