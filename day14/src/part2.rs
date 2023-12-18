use std::collections::HashMap;
use std::fs;

pub fn solve(input: &str) -> u32 {
    let mut grid = fs::read_to_string(input).unwrap();
    let mut weight_indexes: HashMap<u32, Vec<u32>> = Default::default();
    let mut series: Vec<u32> = vec![];
    let mut c: Option<(u32, u32)> = None;
    for i in 0..1000000000 { // it /should/ stop long before getting this high, as soon as a cycle is found!
        grid = cycle(&grid);
        let weight = grid_weight(&grid);
        series.push(weight);
        if weight_indexes.contains_key(&weight) {
            let mut curr = weight_indexes.get(&weight).unwrap().to_owned();
            curr.push(i);
            c = find_cycle(&curr);
            if c.is_some() {
                println!("Found Cycle! {:?}", c.unwrap());
                break;
            }
            weight_indexes.insert(weight, curr);
        } else {
            weight_indexes.insert(weight, vec![i]);
        }
    }
    let cycle_start = c.unwrap().0;
    let cycle_period = c.unwrap().1;
    let pos_in_cycle = (1000000000 - cycle_start) % cycle_period;
    series[cycle_start as usize + pos_in_cycle as usize - 1]
}

fn find_cycle(indexes: &Vec<u32>) -> Option<(u32, u32)> {
    if indexes.len() < 4 {
        None
    } else {
        let mut distances: Vec<u32> = vec![];
        for i in 0..indexes.len() - 1 {
            distances.push(indexes[i+1] - indexes[i]);
        }
        if distances.len() == distances.iter().filter(|e| **e == distances[0]).count() {
            Some((indexes[0], distances[0]))
        } else {
            None
        }
        
    }
}

fn grid_weight(grid: &str) -> u32 {
    let mut total_weight = 0;
    let height = grid.lines().count();
    for (y, line) in grid.lines().enumerate() {
        let distance_from_south = height - y;
        total_weight += distance_from_south as u32 * base_line_weight(line)
    }
    total_weight
}

fn cycle(grid: &str) -> String {
    let step_north = rotate_pattern_right(&tilt_grid_west(&rotate_pattern_left(grid)));
    let step_west = tilt_grid_west(&step_north);
    let step_south = rotate_pattern_left(&tilt_grid_west(&rotate_pattern_right(&step_west)));
    let step_east = tilt_grid_east(&step_south);
    step_east
}

fn base_line_weight(line: &str) -> u32 {
    line.chars().filter(|c| *c == 'O').count() as u32
}

fn tilt_grid_east(grid: &str) -> String {
    let mut tilted_lines: Vec<String> = vec![];
    for line in grid.lines() {
        tilted_lines.push(tilt_line_east(line));
    }
    tilted_lines.join("\n")
}

fn tilt_grid_west(grid: &str) -> String {
    let mut tilted_lines: Vec<String> = vec![];
    for line in grid.lines() {
        tilted_lines.push(tilt_line_west(line));
    }
    tilted_lines.join("\n")
}

fn tilt_line_east(line: &str) -> String {
    tilt_line_west(&line.chars().rev().collect::<String>())
        .chars()
        .rev()
        .collect()
}

fn tilt_line_west(line: &str) -> String {
    let mut res: Vec<(usize, usize)> = vec![];
    let mut curr_pos = 0;
    let mut curr_count = 0;
    for (i, c) in line.chars().enumerate() {
        match c {
            'O' => curr_count += 1,
            '#' => {
                if curr_count > 0 {
                    res.push((curr_pos, curr_count));
                    curr_count = 0;
                }
                curr_pos = i + 1;
            }
            '.' => (),
            _ => panic!("Unexpected char {} in input", c),
        }
    }

    if curr_count > 0 {
        // handle the case where there isn't any # on the line!
        res.push((curr_pos, curr_count));
    }
    let mut new_line = line.to_owned().replace("O", ".");

    for group in res {
        let start = group.0;
        let end = group.0 + group.1;
        new_line.replace_range(start..end, &"O".repeat(group.1));
    }
    new_line
}

fn rotate_pattern_right(pattern: &str) -> String {
    // take a raw grid and rotate it 90° clockwise.
    // why? i can't be bothered having a different function to check for vertical symmetry
    // also vertical symmetry kinda feels more intuitive to implement?
    let old_pattern = pattern
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>(); // turn the grid into a format we can work with
    let mut new_pattern: Vec<Vec<char>> = vec![];

    for _ in 0..old_pattern[0].len() {
        // prepare the receiving array
        new_pattern.push(vec![]);
    }

    for y in 0..old_pattern.len() {
        let act_y = ((old_pattern.len()) - y) - 1;
        for x in 0..old_pattern[act_y].len() {
            new_pattern[x].push(old_pattern[act_y][x]);
        }
    }

    new_pattern
        .iter()
        .map(|line| {
            line.iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join("")
        }) // turn an array of char into a full string representing a line
        .collect::<Vec<String>>()
        .join("\n") // turn an array of lines into the full grid string
}

fn rotate_pattern_left(pattern: &str) -> String {
    // take a raw grid and rotate it 90° clockwise.
    // why? i can't be bothered having a different function to check for vertical symmetry
    // also vertical symmetry kinda feels more intuitive to implement?
    let old_pattern = pattern
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>(); // turn the grid into a format we can work with
    let mut new_pattern: Vec<Vec<char>> = vec![];

    for _ in 0..old_pattern[0].len() {
        // prepare the receiving array
        new_pattern.push(vec![]);
    }

    for y in 0..old_pattern.len() {
        for x in 0..old_pattern[y].len() {
            let act_x = ((old_pattern[y].len()) - x) - 1;
            new_pattern[act_x].push(old_pattern[y][x]);
        }
    }

    new_pattern
        .iter()
        .map(|line| {
            line.iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join("")
        }) // turn an array of char into a full string representing a line
        .collect::<Vec<String>>()
        .join("\n") // turn an array of lines into the full grid string
}

#[cfg(test)]
pub mod test {

    use crate::part2::solve;
    use std::fs;

    use super::cycle;

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 64);
    }

    #[test]
    fn test_cycle() {
        let grid = fs::read_to_string("sample.txt").unwrap();
        let expected_one = fs::read_to_string("sample_cycle_1.txt").unwrap();
        let expected_two = fs::read_to_string("sample_cycle_2.txt").unwrap();
        let expected_three = fs::read_to_string("sample_cycle_3.txt").unwrap();
        let one_cycle = cycle(&grid);
        let two_cycles = cycle(&one_cycle);
        let three_cycles = cycle(&two_cycles);
        assert_eq!(one_cycle, expected_one);
        assert_eq!(two_cycles, expected_two);
        assert_eq!(three_cycles, expected_three);
    }
}
