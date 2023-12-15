use std::fs;
use std::collections::HashSet;

pub fn solve(input: &str) -> u32 {
    let raw_grid: Vec<Vec<char>> = fs::read_to_string(input).unwrap().lines().map(|line| line.chars().collect::<Vec<char>>()).collect();
    let expanded = expand_grid(&raw_grid);
    let galaxies: Vec<(usize, usize)> = get_galaxies(&expanded);
    let mut distances: Vec<u32> = vec![];
    let mut done: HashSet<((usize, usize), (usize, usize))> = Default::default();
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

fn aliased_vector_length(vec: (i32, i32)) -> u32 {
    let abs = abs_vector(vec);
    let repeats = gcd(abs.0, abs.1);
    repeats * (abs.0 / repeats + abs.1 / repeats)
}

fn gcd(a: u32, b: u32) -> u32 {
    if a == 0 {
        b
    } else if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn abs_vector(v: (i32, i32)) -> (u32, u32) {
    (v.0.abs() as u32, v.1.abs() as u32)
}

fn get_vector(ga: (usize, usize), gb: (usize, usize)) -> (i32, i32) {
    (gb.0 as i32 - ga.0 as i32, gb.1 as i32 - ga.1 as i32)
}

fn get_galaxies(grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut scratch = vec![];
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '#' {
                scratch.push((x, y));
            }
        }
    }
    scratch
}

fn expand_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let do_rows = expand_grid_vertical(grid);
    let do_columns = expand_grid_vertical(&rotate_grid_right(&do_rows));
    rotate_grid_left(&do_columns)
}

fn expand_grid_vertical(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> { // only expand rows, rotate grid to expand columns
    let mut new_grid = vec![];
    // rows
    for i in 0..grid.len() {
        if grid[i].iter().filter(|char| **char == '#').count() == 0 {
            new_grid.push(grid[i].to_owned())
        }
        new_grid.push(grid[i].to_owned());
    }
    new_grid
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

fn rotate_grid_left(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_grid = vec![];

    for _ in 0..grid[0].len() { // prepare the receiving array
        new_grid.push(vec![]);
    }

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let act_x = ((grid[y].len()) - x) - 1;
            new_grid[act_x].push(grid[y][x]);
        }
    }

    new_grid
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::part1::{rotate_grid_right, rotate_grid_left, expand_grid, get_galaxies, get_vector, abs_vector, gcd, solve, aliased_vector_length};

    #[test]
    fn expansion() {
        let raw_grid: Vec<Vec<char>> = fs::read_to_string("sample.txt").unwrap().lines().map(|line| line.chars().collect::<Vec<char>>()).collect();
        let expanded_sol: Vec<Vec<char>> = fs::read_to_string("sample_res.txt").unwrap().lines().map(|line| line.chars().collect::<Vec<char>>()).collect();
        let expanded_grid = expand_grid(&raw_grid);
        for line in &expanded_grid {
            println!("{}", line.iter().map(|char| char.to_string()).collect::<Vec<String>>().join(""));
        }
        assert_eq!(expanded_grid.len(), expanded_sol.len());
        for i in 0..expanded_grid.len() {
            println!("Line {i}");
            assert_eq!(expanded_grid[i], expanded_sol[i]);
        }
        assert_eq!(expanded_grid, expanded_sol);
    }

    #[test]
    fn rotate_right() {
        let mut grid:Vec<Vec<char>> = vec![];
        grid.push(vec!['a', 'b']);
        grid.push(vec!['c', 'd']);
        let mut res: Vec<Vec<char>> = vec![];
        res.push(vec!['c', 'a']);
        res.push(vec!['d', 'b']);
        assert_eq!(rotate_grid_right(&grid), res);
    }

    #[test]
    fn rotate_left() {
        let mut grid:Vec<Vec<char>> = vec![];
        grid.push(vec!['a', 'b']);
        grid.push(vec!['c', 'd']);
        let mut res: Vec<Vec<char>> = vec![];
        res.push(vec!['b', 'd']);
        res.push(vec!['a', 'c']);
        assert_eq!(rotate_grid_left(&grid), res);
    }

    #[test]
    fn galaxies() {
        let expanded_grid: Vec<Vec<char>> = fs::read_to_string("sample_res.txt").unwrap().lines().map(|line| line.chars().collect::<Vec<char>>()).collect();
        let mut res = get_galaxies(&expanded_grid);
        let mut check = vec![(4, 0), (9,1), (0,2), (8,5), (1,6), (12,7), (9,10), (0,11), (5,11)];
        res.sort();
        check.sort();
        assert_eq!(res.len(), 9);
        assert_eq!(res, check);
    }

    #[test]
    fn vectors() {
        assert_eq!(get_vector((0, 2), (1, 1)), (1, -1));
        assert_eq!(get_vector((0, 0), (3, 6)), (3, 6));
    }

    #[test]
    fn abs() {
        assert_eq!(abs_vector((1, -1)), (1, 1));
        assert_eq!(abs_vector((3, 6)), (3, 6));
    }

    #[test]
    fn check_gcd() {
        assert_eq!(gcd(3, 6), 3);
        assert_eq!(gcd(270, 192), 6);
        assert_eq!(gcd(192, 270), 6);
    }

    #[test]
    fn length() {
        assert_eq!(aliased_vector_length((3, 3)), 6);
    }

    #[test]
    fn full() {
        assert_eq!(solve("sample.txt"), 374);
    }
}