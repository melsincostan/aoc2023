use std::fs;

pub fn solve(input: &str) -> u32 {
    fs::read_to_string(input)
        .unwrap()
        .split("\n\n") // get individual patterns
        .map(score_pattern)
        .sum()
}

fn score_pattern(pattern: &str) -> u32 {
    let v = find_vertical_symmetry(pattern);
    let h = find_horizontal_symmetry(pattern);
    if v.is_some() {
        v.unwrap() + 1
    } else if h.is_some() {
        (h.unwrap() + 1) * 100
    } else {
        panic!(
            "Should have found a line of symmetry! Pattern below:\n\n{}\n\n",
            pattern
        );
    }
}

fn find_vertical_symmetry(pattern: &str) -> Option<u32> {
    let rotated_pattern = rotate_pattern_right(pattern);
    find_horizontal_symmetry(&rotated_pattern)
}

fn find_horizontal_symmetry(pattern: &str) -> Option<u32> {
    let lines: Vec<&str> = pattern.lines().collect();
    let mut to_check: Vec<usize> = vec![];
    for i in 0..lines.len() - 1 {
        if lines[i] == lines[i + 1] {
            to_check.push(i);
        }
    }

    for pos in to_check {
        let mut valid = true;
        // check lines starting from the potential symmetry line and moving away in both directions each step
        for i in 0..=pos {
            if pos + 1 + i >= lines.len() {
                break;
            }
            if lines[pos - i] != lines[pos + 1 + i] {
                valid = false;
                break;
            }
        }
        if valid {
            return Some(pos as u32);
        }
    }
    None
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

#[cfg(test)]
pub mod test {
    use std::fs;

    use crate::part1::{
        find_horizontal_symmetry, find_vertical_symmetry, rotate_pattern_right, score_pattern, solve,
    };

    #[test]
    fn test_find_horizontal_symmetry() {
        let pattern_hor = fs::read_to_string("part_sample_hor.txt").unwrap();
        let pattern_vert = fs::read_to_string("part_sample_vert.txt").unwrap();
        assert_eq!(find_horizontal_symmetry(&pattern_hor), Some(3));
        assert_eq!(find_horizontal_symmetry(&pattern_vert), None);
    }

    #[test]
    fn test_find_vertical_symmetry() {
        let pattern_vert = fs::read_to_string("part_sample_vert.txt").unwrap();
        let pattern_hor = fs::read_to_string("part_sample_hor.txt").unwrap();
        assert_eq!(find_vertical_symmetry(&pattern_vert), Some(4));
        assert_eq!(find_vertical_symmetry(&pattern_hor), None);
    }

    #[test]
    fn test_rotate_pattern_right() {
        let pattern = "ab\ncd";
        assert_eq!(rotate_pattern_right(pattern), "ca\ndb");
    }

    #[test]
    fn test_score_pattern() {
        let pattern_vert = fs::read_to_string("part_sample_vert.txt").unwrap();
        let pattern_hor = fs::read_to_string("part_sample_hor.txt").unwrap();
        assert_eq!(score_pattern(&pattern_vert), 5);
        assert_eq!(score_pattern(&pattern_hor), 400);
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 405);
    }
}
