use std::fmt::Display;
use std::fs::File;
use std::io::{prelude::*, BufReader, Lines};

pub fn get_lines_from_file(day: &str) -> Lines<BufReader<File>> {
    let file = File::open(format!("{}/input.txt", day)).expect("Could not open file");
    let reader = BufReader::new(file);
    reader.lines()
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
