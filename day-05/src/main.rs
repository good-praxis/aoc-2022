use common::*;

fn main() {
    fn fill_initial_stacks(input: &[String], stacks: &mut [Vec<char>]) {
        for (i, stack) in stacks.iter_mut().enumerate() {
            for j in (0..8).rev() {
                let char = input[j].chars().nth(1 + (i * 4)).unwrap();
                if !char.is_whitespace() {
                    stack.push(char);
                }
            }
        }
    }

    fn get_amount_source_dest(instructions: &str) -> (usize, usize, usize) {
        let split_line = instructions.split(' ').collect::<Vec<&str>>();
        let [amount, source, dest] = [split_line[1], split_line[3], split_line[5]]
            .map(str::parse::<usize>)
            .map(Result::unwrap);

        (amount, source - 1, dest - 1)
    }

    fn walk_through_9000_instructions(input: &[String], stacks: &mut [Vec<char>]) {
        for line in input.iter().skip(10) {
            let (amount, source, dest) = get_amount_source_dest(line);
            for _i in 0..amount {
                if let Some(item) = stacks[source].pop() {
                    stacks[dest].push(item);
                }
            }
        }
    }

    fn walk_through_9001_instructions(input: &[String], stacks: &mut [Vec<char>]) {
        for line in input.iter().skip(10) {
            let (amount, source, dest) = get_amount_source_dest(line);
            let offset = stacks[source].len().saturating_sub(amount);
            let drain = stacks[source].drain(offset..).collect::<Vec<char>>();
            stacks[dest].extend(drain);
        }
    }

    fn get_topmost(stacks: &[Vec<char>]) -> String {
        stacks.iter().fold(String::new(), |mut acc, stack| {
            acc.push(*stack.last().unwrap());
            acc
        })
    }

    let input = get_line_vec_from_file("day-05");
    let mut stacks: Vec<Vec<char>> = vec![vec![]; 9];

    fill_initial_stacks(&input, &mut stacks);
    let mut sim_stacks = stacks.clone();

    walk_through_9000_instructions(&input, &mut sim_stacks);
    let sim_topmost = get_topmost(&sim_stacks);
    walk_through_9001_instructions(&input, &mut stacks);
    let topmost = get_topmost(&stacks);

    present_result(sim_topmost, Some(topmost));
}
