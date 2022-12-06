use common::*;
use std::collections::HashSet;

fn main() {
    let input = get_line_vec_from_file("day-06")[0].clone();

    let start_of_package = get_unique_sequence_position(&input, 4);
    let start_of_message = get_unique_sequence_position(&input, 14);

    present_result(start_of_package, Some(start_of_message))
}

fn get_unique_sequence_position(input: &str, len: usize) -> usize {
    let mut set: HashSet<u8> = HashSet::new();
    let mut counter = len;

    for slice in input.as_bytes().windows(len) {
        set.extend(slice.iter());
        if set.len() == len {
            break;
        };
        counter += 1;
        set.clear();
    }

    counter
}
