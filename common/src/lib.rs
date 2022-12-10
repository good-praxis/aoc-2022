use std::fmt::Display;
use std::fs::File;
use std::io::{prelude::*, BufReader, Error};

pub fn get_lines_from_file(day: &str) -> impl Iterator<Item = Result<String, Error>> {
    let file = File::open(format!("{}/input.txt", day)).expect("Could not open file");
    let reader = BufReader::new(file);
    reader.lines()
}

pub fn get_unwraped_lines_from_file(day: &str) -> impl Iterator<Item = String> {
    get_lines_from_file(day).map(Result::unwrap)
}

pub fn get_line_vec_from_file(day: &str) -> Vec<String> {
    get_unwraped_lines_from_file(day).collect::<Vec<String>>()
}

pub fn present_result<T>(part1: T, part2: Option<T>)
where
    T: Display,
{
    println!("part 1: {}", part1);
    if let Some(part2) = part2 {
        println!("part 2: {}", part2);
    }
}
