use std::fs::File;
use std::io::prelude::*;
use std::num::ParseIntError;
use anyhow::Result;

#[macro_use]
extern crate lazy_static;

pub fn read_nums(filename: &str) -> Result<Vec<isize>> {
    let mut file = File::open(filename)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    let iter = input.split_whitespace();
    let x : std::result::Result<Vec<isize>, ParseIntError> = iter.map(|x| x.parse::<isize>()).collect();
    Ok(x?) 
}

use std::io::{Lines, BufReader};

pub fn read_lines(filename: &str) -> Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    let f = BufReader::new(file);
    Ok(f.lines())
}

pub mod day1;
pub mod day2;
pub mod day3;
