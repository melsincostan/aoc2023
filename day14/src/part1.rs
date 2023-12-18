use std::fs;

pub fn solve(input: &str) -> u32 {
    let tilted_grid = tilt_grid_north(&fs::read_to_string(input).unwrap());
    let height = tilted_grid.lines().count();
    let mut total_weight = 0;
    for (y, line) in tilted_grid.lines().enumerate() {
        let distance_from_south = height - y;
        total_weight += distance_from_south as u32 * base_line_weight(line)
    }
    total_weight
}

fn base_line_weight(line: &str) -> u32 {
    line.chars().filter(|c| *c == 'O').count() as u32
}

fn tilt_grid_north(grid: &str) -> String {
    let rotated_grid = rotate_pattern_left(grid);
    rotate_pattern_right(&rotated_grid.lines().map(tilt_line_west).collect::<Vec<String>>().join("\n"))
}

fn tilt_line_west(line: &str) -> String{
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
                curr_pos = i+1;
            },
            '.' => (),
            _ => panic!("Unexpected char {} in input", c)
        }
    }

    if curr_count > 0 { // handle the case where there isn't any # on the line!
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

    use std::fs;
    use crate::part1::{rotate_pattern_left, rotate_pattern_right, tilt_line_west, tilt_grid_north, base_line_weight, solve};

    #[test]
    fn test_rotate_pattern_left() {
        assert_eq!(rotate_pattern_left("ab\ncd"), "bd\nac");
    }

    #[test]
    fn test_rotate_pattern_right() {
        assert_eq!(rotate_pattern_right("ab\ncd"), "ca\ndb");
    }

    #[test]
    fn test_tilt_line_west() {
        assert_eq!(tilt_line_west(".O...#O..O"), "O....#OO..");
        assert_eq!(tilt_line_west("...OO....O"), "OOO.......");
        assert_eq!(tilt_line_west("OO.O.O..##"), "OOOO....##");
    }

    #[test]
    fn test_tilt_grid_north() {
        let grid = fs::read_to_string("sample.txt").unwrap();
        let expected_result = fs::read_to_string("sample_slid.txt").unwrap();
        assert_eq!(tilt_grid_north(&grid), expected_result);
    }

    #[test]
    fn test_base_line_weight() {
        assert_eq!(base_line_weight("OOOO.#.O.."), 5);
        assert_eq!(base_line_weight("........#."), 0);
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 136);
    }
}