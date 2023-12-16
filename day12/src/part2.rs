use std::fs;
use std::collections::{HashMap};

#[derive(Debug, PartialEq, Hash, Eq)]
struct SpringLine {
    springs: Vec<SpringState>,
    groups: Vec<u64>,
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

pub fn solve(input: &str) -> u64 {
    fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(unfold_line)
        .map(parse_line)
        .map(possible_solutions)
        .sum()
}

fn possible_solutions(line: SpringLine) -> u64 {
    let mut cache: HashMap<(Vec<SpringState>, Vec<u64>), u64> = Default::default();
    recurs_possible_solutions(&line.springs, &line.groups, &mut cache)
}

fn recurs_possible_solutions(springs: &Vec<SpringState>, groups: &Vec<u64>, cache: &mut HashMap<(Vec<SpringState>, Vec<u64>), u64>) -> u64 {
    if cache.contains_key(&(springs.to_owned(), groups.to_owned())) {
        return *cache.get(&(springs.to_owned(), groups.to_owned())).unwrap();
    } else {
        if springs.is_empty() {
            if groups.is_empty() {
                cache.insert((springs.to_owned(), groups.to_owned()), 1);
                1
            } else {
                cache.insert((springs.to_owned(), groups.to_owned()), 0);
                0
            }
        } else if groups.is_empty() {
            if springs.contains(&SpringState::Damaged) {
                cache.insert((springs.to_owned(), groups.to_owned()), 0);
                0
            } else {
                cache.insert((springs.to_owned(), groups.to_owned()), 1);
                1
            }
        } else {
            let mut result = 0;
            if springs[0] != SpringState::Damaged {
                result += recurs_possible_solutions(&springs[1..].to_vec(), groups, cache)
            }
    
            if springs[0] != SpringState::Operational {
                if groups[0] <= springs.len() as u64
                    && !springs[..groups[0] as usize].contains(&SpringState::Operational)
                    && (groups[0] == springs.len() as u64
                        || springs[groups[0] as usize] != SpringState::Damaged)
                {
                    let nsprings = if groups[0] as usize + 1 >= springs.len() {
                        // don't go trying to index starting at something too big...
                        vec![]
                    } else {
                        springs[groups[0] as usize + 1..].to_vec()
                    };
                    result += recurs_possible_solutions(&nsprings, &groups[1..].to_vec(), cache);
                }
            }
            cache.insert((springs.to_owned(), groups.to_owned()), result);
            result
        }
    }

}

fn unfold_line(line: &str) -> String {
    let springs = vec![line.split_whitespace().next().unwrap(); 5];
    let groups = vec![line.split_whitespace().last().unwrap(); 5];
    format!("{} {}", springs.join("?"), groups.join(","))
}

fn parse_line(line: String) -> SpringLine {
    let springs: Vec<SpringState> = line
        .split_whitespace()
        .next()
        .unwrap()
        .chars()
        .map(state_from_char)
        .collect();
    let groups: Vec<u64> = line
        .split_whitespace()
        .last()
        .unwrap()
        .split(",")
        .map(|num| num.parse::<u64>().unwrap()) // parse str numbers to actual numbers
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

    use crate::part2::{solve, unfold_line};

    #[test]
    fn test_unfold_line() {
        assert_eq!(unfold_line(".# 1"), ".#?.#?.#?.#?.# 1,1,1,1,1");
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 525152);
    }
}
