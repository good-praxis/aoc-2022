use common::*;
use std::collections::HashSet;

type Coords = (i16, i16);

fn main() {
    let mut knots: [Coords; 10] = [(0, 0); 10];

    let mut first_knot_set: HashSet<Coords> = HashSet::new();
    first_knot_set.insert(knots[1]);
    let mut last_knot_set: HashSet<Coords> = first_knot_set.clone();

    for line in get_line_vec_from_file("day-09").into_iter() {
        let instructions = line.split(' ').collect::<Vec<&str>>();
        let direction = get_direction(instructions[0]);
        let steps = instructions[1].parse::<u16>().unwrap();
        for _i in 0..steps {
            knots[0] = move_head_into_direction(&knots[0], &direction);
            for i in 1..knots.len() {
                knots[i] = move_tail_if_you_feel_like_it(&knots[i], &knots[i - 1]);
            }
            first_knot_set.insert(knots[1]);
            last_knot_set.insert(knots[9]);
        }
    }
    present_result(first_knot_set.len(), Some(last_knot_set.len()));
}

enum Direction {
    Right,
    Left,
    Up,
    Down,
}
use Direction::*;
fn get_direction(dir: &str) -> Direction {
    match dir {
        "R" => Right,
        "L" => Left,
        "U" => Up,
        "D" => Down,
        _ => panic!("Directions unclear, got stuck in washing machine"),
    }
}
fn move_head_into_direction(head: &Coords, dir: &Direction) -> Coords {
    match dir {
        Up => (head.0, head.1 + 1),
        Down => (head.0, head.1 - 1),
        Left => (head.0 - 1, head.1),
        Right => (head.0 + 1, head.1),
    }
}
fn move_tail_if_you_feel_like_it(tail: &Coords, head: &Coords) -> Coords {
    if head.0.abs_diff(tail.0) <= 1 && head.1.abs_diff(tail.1) <= 1 {
        return *tail;
    }

    let x_dir = (head.0 - tail.0).clamp(-1, 1);
    let y_dir = (head.1 - tail.1).clamp(-1, 1);

    (tail.0 + x_dir, tail.1 + y_dir)
}
