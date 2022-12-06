use common::*;
use std::collections::{HashSet, VecDeque};

fn main() {
    let input = get_line_vec_from_file("day-06")[0].clone();

    let start_of_package = get_unique_sequence_position(&input, 4);
    let start_of_message = get_unique_sequence_position(&input, 14);

    present_result(start_of_package, Some(start_of_message))
}

fn get_unique_sequence_position(input: &String, len: usize) -> usize {
    let mut history = VecDeque::new();
    let mut set: HashSet<char> = HashSet::new();
    let mut counter = 0;

    for c in input.chars() {
        counter += 1;
        if history.len() >= len {
            history.pop_front();
        }
        history.push_back(c);
        set.clear();
        set.extend(history.iter());
        if set.len() == len {
            break;
        };
    }

    counter
}
