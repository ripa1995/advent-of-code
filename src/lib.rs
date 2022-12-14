#![allow(dead_code)]
use std::fs::File;
pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;

pub fn open_file(path: String) -> File {
    File::open(path).expect("Failed to read the file...")
}
