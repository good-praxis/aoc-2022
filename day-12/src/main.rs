use pathfinding::prelude::{bfs, Matrix};

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

type Pos = (usize, usize);
fn part1(input: &str) -> usize {
    let (matrix, start, end) = &parse(input);
    bfs(
        start,
        |&pos| {
            matrix
                .neighbours(pos, false)
                .filter(move |n_pos| matrix[*n_pos] <= matrix[pos] + 1)
        },
        |&pos| pos == *end,
    )
    .unwrap()
    .len()
        - 1
}

fn part2(input: &str) -> usize {
    let (matrix, _start, start) = &parse(input);
    bfs(
        start,
        |&pos| {
            matrix
                .neighbours(pos, false)
                .filter(move |&n_pos| matrix[pos] <= matrix[n_pos] + 1)
        },
        |&pos| matrix[pos] == b'a',
    )
    .unwrap()
    .len()
        - 1
}

fn parse(input: &str) -> (Matrix<u8>, Pos, Pos) {
    let mut matrix = Matrix::from_rows(input.lines().map(str::bytes)).unwrap();
    let start = matrix.indices().find(|&pos| matrix[pos] == b'S').unwrap();
    matrix[start] = b'a';
    let end = matrix.indices().find(|&pos| matrix[pos] == b'E').unwrap();
    matrix[end] = b'z';
    (matrix, start, end)
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(TEST_INPUT), 31);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(TEST_INPUT), 29);
    }
}
