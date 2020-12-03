extern crate aoc_2020;

use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error};
use std::result::Result;

use aoc_2020::day1::find_three;

fn main() -> Result<(), Error> {
    let input = "input/day1_a.txt";
    let numbers = convert_file_contents_to_numbers(&input)?;
    let result = find_three(numbers, 2020);
    println!("{}", result);
    Ok(())
}

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
