use common::*;
use std::ops::RangeInclusive;

fn main() {
    fn get_ranges_from_line(str: &str) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
        // i-j,k-l
        let ranges = str.split(',');
        let ranges = ranges
            .map(|range| {
                range
                    .split('-')
                    .map(str::parse::<usize>)
                    .map(Result::unwrap)
                    .collect::<Vec<usize>>()
            })
            .map(|range| get_range((range[0], range[1])))
            .collect::<Vec<RangeInclusive<usize>>>();
        (ranges[0].clone(), ranges[1].clone())
    }
    fn get_range(from: (usize, usize)) -> RangeInclusive<usize> {
        from.0..=from.1
    }

    let full_overlap = get_lines_from_file("day-04").fold(0, |acc, line| {
        let line = line.unwrap();
        let (first_range, second_range) = get_ranges_from_line(&line);
        if first_range.contains(second_range.start()) && first_range.contains(second_range.end())
            || second_range.contains(first_range.start())
                && second_range.contains(first_range.end())
        {
            return acc + 1;
        }
        acc
    });

    let any_overlap = get_lines_from_file("day-04").fold(0, |acc, line| {
        let line = line.unwrap();
        let (mut first_range, second_range) = get_ranges_from_line(&line);
        if first_range.any(|i| second_range.contains(&i)) {
            return acc + 1;
        }
        acc
    });

    present_result(full_overlap, Some(any_overlap));
}
