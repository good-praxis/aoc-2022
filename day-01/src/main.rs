use common::*;

fn main() {
    let mut calories: Vec<u32> = Vec::new();
    let mut temp = 0;

    for line in get_lines_from_file("day-01") {
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
    let all_the_cals = calories[calories.len() - 3..].iter().sum::<u32>();

    present_result(biggest_meal, Some(&all_the_cals));
}
