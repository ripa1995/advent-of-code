use std::fs::File;
pub mod day1;
pub mod day2;

pub fn open_file(path: String) -> File {
    File::open(path).expect("Failed to read the file...")
}