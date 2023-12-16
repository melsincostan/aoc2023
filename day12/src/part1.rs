use std::fs;

#[derive(Debug, PartialEq)]
struct SpringLine {
    springs: Vec<SpringState>,
    groups: Vec<u32>,
}

#[derive(Debug, PartialEq, Clone)]
enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

pub fn solve(input: &str) -> u32 {
    fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(parse_line)
        .map(possible_solutions)
        .sum()
}

fn possible_solutions(line: SpringLine) -> u32 {
    recurs_possible_solutions(&line.springs, &line.groups)
}

fn recurs_possible_solutions(springs: &Vec<SpringState>, groups: &Vec<u32>) -> u32 {
    if springs.is_empty() {
        if groups.is_empty() {
            1
        } else {
            0
        }
    } else if groups.is_empty() {
        if springs.contains(&SpringState::Damaged) {
            0
        } else {
            1
        }
    } else {
        let mut result = 0;
        if springs[0] != SpringState::Damaged {
            result += recurs_possible_solutions(&springs[1..].to_vec(), groups)
        }

        if springs[0] != SpringState::Operational {
            if groups[0] <= springs.len() as u32
                && !springs[..groups[0] as usize].contains(&SpringState::Operational)
                && (groups[0] == springs.len() as u32
                    || springs[groups[0] as usize] != SpringState::Damaged)
            {
                let nsprings = if groups[0] as usize + 1 >= springs.len() {
                    // don't go trying to index starting at something too big...
                    vec![]
                } else {
                    springs[groups[0] as usize + 1..].to_vec()
                };
                result += recurs_possible_solutions(&nsprings, &groups[1..].to_vec());
            }
        }
        result
    }
}

fn groups_from_line(line: &Vec<SpringState>) -> Vec<u32> {
    let mut count = 0;
    let mut in_group = false;
    let mut groups = vec![];
    for spring in line {
        if *spring == SpringState::Damaged {
            if in_group {
                count += 1;
            } else {
                count = 1;
                in_group = true;
            }
        } else {
            if in_group {
                in_group = false;
                groups.push(count);
            }
        }
    }

    // if the line ends in a group, add it now!
    // it won't have been added yet since the loop waits for the next undamaged spring to add to the group!
    if in_group {
        groups.push(count);
    }
    groups
}

fn parse_line(line: &str) -> SpringLine {
    let springs: Vec<SpringState> = line
        .split_whitespace()
        .next()
        .unwrap()
        .chars()
        .map(state_from_char)
        .collect();
    let groups: Vec<u32> = line
        .split_whitespace()
        .last()
        .unwrap()
        .split(",")
        .map(|num| num.parse::<u32>().unwrap()) // parse str numbers to actual numbers
        .collect();
    SpringLine {
        springs: springs,
        groups: groups,
    }
}

fn state_from_char(c: char) -> SpringState {
    match c {
        '.' => SpringState::Operational,
        '#' => SpringState::Damaged,
        '?' => SpringState::Unknown,
        _ => panic!("Invalid status: {}", c),
    }
}

#[cfg(test)]
mod test {

    use crate::part1::{
        groups_from_line, parse_line, possible_solutions, solve, state_from_char, SpringLine,
        SpringState,
    };

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("???.### 1,1,3"),
            SpringLine {
                springs: vec![
                    SpringState::Unknown,
                    SpringState::Unknown,
                    SpringState::Unknown,
                    SpringState::Operational,
                    SpringState::Damaged,
                    SpringState::Damaged,
                    SpringState::Damaged
                ],
                groups: vec![1, 1, 3],
            }
        );

        assert_eq!(
            parse_line(".??..??...?##. 1,1,3"),
            SpringLine {
                springs: vec![
                    SpringState::Operational,
                    SpringState::Unknown,
                    SpringState::Unknown,
                    SpringState::Operational,
                    SpringState::Operational,
                    SpringState::Unknown,
                    SpringState::Unknown,
                    SpringState::Operational,
                    SpringState::Operational,
                    SpringState::Operational,
                    SpringState::Unknown,
                    SpringState::Damaged,
                    SpringState::Damaged,
                    SpringState::Operational
                ],
                groups: vec![1, 1, 3],
            }
        );

        assert_eq!(
            parse_line("?#?#?#?#?#?#?#? 1,3,1,6"),
            SpringLine {
                springs: vec![
                    SpringState::Unknown,
                    SpringState::Damaged,
                    SpringState::Unknown,
                    SpringState::Damaged,
                    SpringState::Unknown,
                    SpringState::Damaged,
                    SpringState::Unknown,
                    SpringState::Damaged,
                    SpringState::Unknown,
                    SpringState::Damaged,
                    SpringState::Unknown,
                    SpringState::Damaged,
                    SpringState::Unknown,
                    SpringState::Damaged,
                    SpringState::Unknown
                ],
                groups: vec![1, 3, 1, 6],
            }
        );
    }

    #[test]
    fn test_groups_from_line() {
        let line1 = "#.#.###".chars().map(state_from_char).collect();
        let line2 = ".#.###.#.######".chars().map(state_from_char).collect();
        let line3 = "???.###".chars().map(state_from_char).collect();
        assert_eq!(groups_from_line(&line1), vec![1, 1, 3]);
        assert_eq!(groups_from_line(&line2), vec![1, 3, 1, 6]);
        assert_eq!(groups_from_line(&line3), vec![3]);
    }

    #[test]
    fn test_possible_solutions() {
        let line1 = parse_line("???.### 1,1,3");
        let line2 = parse_line(".??..??...?##. 1,1,3");
        let line3 = parse_line("?###???????? 3,2,1");

        assert_eq!(possible_solutions(line1), 1);
        assert_eq!(possible_solutions(line2), 4);
        assert_eq!(possible_solutions(line3), 10);
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 21);
    }
}
