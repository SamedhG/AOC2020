use std::fs::File;
use std::io::prelude::*;
use std::num::ParseIntError;
use anyhow::Result;


pub fn read_nums(filename: &str) -> Result<Vec<isize>> {
    let mut file = File::open(filename)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    let iter = input.split_whitespace();
    let x : std::result::Result<Vec<isize>, ParseIntError> = iter.map(|x| x.parse::<isize>()).collect();
    Ok(x?) 
}

pub mod day1;
