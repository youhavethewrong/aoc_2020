extern crate aoc_2020;

use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error};
use std::result::Result;

use aoc_2020::day1::find_2020;

fn main() -> Result<(), Error> {
    let file = File::open("input/day1_a.txt")?;
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

    let result = find_2020(numbers);
    println!("{}", result);
    Ok(())
}
