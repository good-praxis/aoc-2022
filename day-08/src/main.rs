use common::*;
use std::collections::HashSet;

fn main() {
    let lines = get_line_vec_from_file("day-08");
    let grid: Vec<Vec<u32>> = lines
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    let column_len = grid[1].len();
    let mut visible_set: HashSet<(usize, usize)> = HashSet::new();
    let mut max_score = 0;
    for total in 0..(grid.len() * column_len) {
        let row_idx = total / grid.len();
        let col_idx = total % grid.len();
        if is_visible((row_idx, col_idx), &grid) {
            visible_set.insert((row_idx, col_idx));
        }
        max_score = max_score.max(calc_scenic_score((row_idx, col_idx), &grid));
    }

    let visible = visible_set.len();
    present_result(visible, Some(max_score));
}
fn is_visible(idx: (usize, usize), grid: &Vec<Vec<u32>>) -> bool {
    let value = grid[idx.0][idx.1];
    idx.0 == 0
        || idx.1 == 0
        || idx.0 == grid.len() - 1
        || idx.1 == grid[0].len() - 1
        || (0..idx.0).all(|row_idx| value > grid[row_idx][idx.1])
        || (0..idx.1).all(|col_idx| value > grid[idx.0][col_idx])
        || ((idx.0 + 1)..grid.len()).all(|row_idx| value > grid[row_idx][idx.1])
        || ((idx.1 + 1)..grid[0].len()).all(|col_idx| value > grid[idx.0][col_idx])
}
fn calc_scenic_score(idx: (usize, usize), grid: &[Vec<u32>]) -> usize {
    get_score_up(idx, grid)
        * get_score_down(idx, grid)
        * get_score_left(idx, grid)
        * get_score_right(idx, grid)
}
fn get_score_up(idx: (usize, usize), grid: &[Vec<u32>]) -> usize {
    let mut local_score = 0;
    let val = grid[idx.0][idx.1];
    for row_idx in (0..idx.0).rev() {
        local_score += 1;
        if grid[row_idx][idx.1] >= val {
            break;
        }
    }
    local_score
}
fn get_score_down(idx: (usize, usize), grid: &[Vec<u32>]) -> usize {
    let mut local_score = 0;
    let val = grid[idx.0][idx.1];
    for row in grid.iter().skip(idx.0 + 1) {
        local_score += 1;
        if row[idx.1] >= val {
            break;
        }
    }
    local_score
}
fn get_score_left(idx: (usize, usize), grid: &[Vec<u32>]) -> usize {
    let mut local_score = 0;
    let val = grid[idx.0][idx.1];
    for col_idx in (0..idx.1).rev() {
        local_score += 1;
        if grid[idx.0][col_idx] >= val {
            break;
        }
    }
    local_score
}
fn get_score_right(idx: (usize, usize), grid: &[Vec<u32>]) -> usize {
    let mut local_score = 0;
    let val = grid[idx.0][idx.1];
    for col_idx in idx.1 + 1..grid[0].len() {
        local_score += 1;
        if grid[idx.0][col_idx] >= val {
            break;
        }
    }
    local_score
}
