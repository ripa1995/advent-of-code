use std::{
    collections::VecDeque,
    fs::File,
    io::{self, BufRead},
    str::FromStr,
};

use crate::open_file;

#[derive(Default)]
struct DirectoryInfo {
    pub name: String,
    pub files: Vec<FileInfo>,
    pub sub_dir: Vec<DirectoryInfo>,
}

#[derive(Default)]
struct FileInfo {
    pub name: String,
    pub size: u64,
}

pub fn calculate_sum() {
    let file = open_file(
        String::from_str("./inputs/day7.txt").expect("Not a string..."),
    );

    let dir = build_directories(file);
    let mut cumulated_sum = 0;
    sum_directories_with_size_less_than(&dir, 100000, &mut cumulated_sum);
    println!("{}", cumulated_sum);
}

pub fn get_directory_to_be_deleted() {
    let file = open_file(
        String::from_str("./inputs/day7.txt").expect("Not a string..."),
    );

    let dir = build_directories(file);
    let res = sum_directories_with_size_less_than(&dir, u64::MAX, &mut 0);
    let currently_free = 70000000 - res;
    let need_to_free = 30000000 - currently_free;
    let mut directories_size = Vec::new();
    get_directory_could_be_deleted(&dir, need_to_free, &mut directories_size);
    directories_size.sort();
    println!("{:?}", directories_size.first());
}

fn build_directory(
    lines: &mut VecDeque<Result<String, std::io::Error>>,
    dir_name: String,
) -> DirectoryInfo {
    let mut directory = DirectoryInfo {
        name: dir_name,
        ..Default::default()
    };
    while let Some(val) = lines.pop_front() {
        if let Ok(val) = val {
            let mut splitted = val.split(' ');
            if let Some(part) = splitted.next() {
                match part {
                    "$" => {
                        //operation
                        match (splitted.next(), splitted.next()) {
                            (Some("cd"), Some("..")) => return directory,
                            (Some("cd"), Some(dir_name)) => {
                                let sub_dir = build_directory(
                                    lines,
                                    dir_name.to_string(),
                                );
                                directory.sub_dir.push(sub_dir);
                            }
                            _ => {}
                        }
                    }
                    other => {
                        //check if number
                        if let Ok(file_size) = other.parse::<u64>() {
                            if let Some(file_name) = splitted.next() {
                                let file_info = FileInfo {
                                    name: file_name.to_string(),
                                    size: file_size,
                                };
                                directory.files.push(file_info);
                            }
                        }
                    }
                }
            }
        }
    }
    directory
}

fn build_directories(file: File) -> DirectoryInfo {
    let mut base_dir = DirectoryInfo::default();

    let mut lines: VecDeque<_> = io::BufReader::new(file).lines().collect();

    if let Some(_first) = lines.pop_front() {
        base_dir = build_directory(&mut lines, "/".to_string())
    }

    base_dir
}

fn sum_directories_with_size_less_than(
    dir: &DirectoryInfo,
    limit: u64,
    cumulated_sum: &mut u64,
) -> u64 {
    let files_size: u64 = dir.files.iter().map(|x| x.size).sum();
    let mut directories_size = 0;
    for directory in &dir.sub_dir {
        directories_size += sum_directories_with_size_less_than(
            directory,
            limit,
            cumulated_sum,
        );
    }
    let current_dir_size = files_size + directories_size;
    if current_dir_size <= limit {
        *cumulated_sum += current_dir_size;
    }
    current_dir_size
}

fn get_directory_could_be_deleted(
    dir: &DirectoryInfo,
    required: u64,
    list_directories_size: &mut Vec<u64>,
) -> u64 {
    let files_size: u64 = dir.files.iter().map(|x| x.size).sum();
    let mut directories_size = 0;
    for directory in &dir.sub_dir {
        directories_size += get_directory_could_be_deleted(
            directory,
            required,
            list_directories_size,
        );
    }
    let current_dir_size = files_size + directories_size;
    if current_dir_size > required {
        list_directories_size.push(current_dir_size)
    }
    current_dir_size
}
