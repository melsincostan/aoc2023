use std::fs;
use std::collections::HashSet;

pub fn solve(input: &str, expansion: usize) -> u64 {
    let raw_grid: Vec<Vec<char>> = fs::read_to_string(input).unwrap().lines().map(|line| line.chars().collect::<Vec<char>>()).collect();
    let exp_rows = expand_grid_vertical(&raw_grid);
    let exp_cols = expand_grid_vertical(&rotate_grid_right(&raw_grid));
    let galaxies: Vec<(u64, u64)> = get_galaxies(&raw_grid, &exp_rows, &exp_cols, expansion as u64 - 1);
    let mut distances: Vec<u64> = vec![];
    let mut done: HashSet<((u64, u64), (u64, u64))> = Default::default();
    for ga in &galaxies {
        for gb in &galaxies {
            if *ga != *gb && !done.contains(&(*ga, *gb)){
                distances.push(aliased_vector_length(get_vector(*ga, *gb)));
                done.insert((*gb, *ga)); // if we've done a, b we don't want to to b, a
            }
        }
    }
    distances.iter().sum()
}

fn aliased_vector_length(vec: (i64, i64)) -> u64 {
    let abs = abs_vector(vec);
    let repeats = gcd(abs.0, abs.1);
    repeats * (abs.0 / repeats + abs.1 / repeats)
}

fn gcd(a: u64, b: u64) -> u64 {
    if a == 0 {
        b
    } else if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn abs_vector(v: (i64, i64)) -> (u64, u64) {
    (v.0.abs() as u64, v.1.abs() as u64)
}

fn get_vector(ga: (u64, u64), gb: (u64, u64)) -> (i64, i64) {
    (gb.0 as i64 - ga.0 as i64, gb.1 as i64 - ga.1 as i64)
}

fn get_galaxies(grid: &Vec<Vec<char>>, exp_rows: &Vec<usize>, exp_cols: &Vec<usize>, expansion: u64) -> Vec<(u64, u64)> {
    let mut scratch = vec![];
    for y in 0..grid.len() {
        let y_offset = exp_rows.iter().filter(|num| **num <= y).count() as u64 * expansion;
        for x in 0..grid[y].len() {
            let x_offset = exp_cols.iter().filter(|num| **num <= x).count() as u64 * expansion;
            if grid[y][x] == '#' {
                scratch.push((x as u64 + x_offset, y as u64 + y_offset));
            }
        }
    }
    scratch
}

fn expand_grid_vertical(grid: &Vec<Vec<char>>) -> Vec<usize> { // only expand rows, rotate grid to expand columns
    let mut res = vec![];
    // rows
    for i in 0..grid.len() {
        if grid[i].iter().filter(|char| **char == '#').count() == 0 {
            res.push(i);
        }
    }
    res
}

fn rotate_grid_right(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_grid = vec![];

    for _ in 0..grid[0].len() { // prepare the receiving array
        new_grid.push(vec![]);
    }

    for y in 0..grid.len() {
        let act_y = ((grid.len()) - y) - 1;
        for x in 0..grid[act_y].len() {
            new_grid[x].push(grid[act_y][x]);
        }
    }

    new_grid
}

#[cfg(test)]
mod test {
    use crate::part2::solve;

    #[test]
    fn full() {
        assert_eq!(solve("sample.txt", 2), 374);
        assert_eq!(solve("sample.txt", 10), 1030);
        assert_eq!(solve("sample.txt", 100), 8410);
    }
}