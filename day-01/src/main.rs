use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("day-01/input.txt").expect("Could not open file");
    let reader = BufReader::new(file);

    let mut calories: Vec<u32> = Vec::new();
    let mut temp = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() || line == "\n" {
            calories.push(temp);
            temp = 0;
            continue;
        }
        temp += line.parse::<u32>().unwrap();
    }
    calories.sort();
    let biggest_meal = calories.last().unwrap();
    println!("part 1: {:?}", biggest_meal);

    let all_the_cals = calories[calories.len() - 3..].into_iter().sum::<u32>();
    println!("part 2: {:?}", all_the_cals);
}
