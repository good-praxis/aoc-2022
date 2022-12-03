use common::*;
use std::collections::HashSet;

fn main() {
    fn set_from_string(compartment: &str) -> HashSet<usize> {
        let mut set = HashSet::new();
        let priority_vec = built_priority_vec();

        set.extend(
            compartment
                .chars()
                .map(|item| get_priority(item, &priority_vec)),
        );
        set
    }

    fn built_priority_vec() -> Vec<char> {
        let mut vec = Vec::with_capacity(52);
        vec.extend((b'a'..=b'z').map(char::from));
        vec.extend((b'A'..=b'Z').map(char::from));
        vec
    }

    fn get_priority(c: char, v: &[char]) -> usize {
        v.iter()
            .position(|&item| c == item)
            .expect("Invalid letter")
            + 1
    }

    let compartment_priority = get_lines_from_file("day-03").fold(0, |acc, line| {
        let line = line.unwrap();
        let mid = line.len() / 2;
        let (compartment_one, compartment_two) = line.split_at(mid);

        let first_set = set_from_string(compartment_one);
        let second_set = set_from_string(compartment_two);
        acc + first_set.intersection(&second_set).sum::<usize>()
    });

    let badge_priority = get_lines_from_file("day-03")
        .map(Result::unwrap)
        .collect::<Vec<String>>()
        .chunks(3)
        .fold(0, |acc, chunk| {
            let first_set = set_from_string(&chunk[0]);
            let second_set = set_from_string(&chunk[1]);
            let third_set = set_from_string(&chunk[2]);

            acc + first_set
                .intersection(&second_set)
                .cloned()
                .collect::<HashSet<usize>>()
                .intersection(&third_set)
                .sum::<usize>()
        });

    present_result(compartment_priority, Some(badge_priority))
}
