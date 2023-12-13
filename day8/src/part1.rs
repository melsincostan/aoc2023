use std::fs;
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
}

struct Children {
    left: String,
    right: String,
}

#[derive(PartialEq, Debug)]
enum Step {
    Right,
    Left
}

pub fn solve(input: &str) -> u32 {
    let file = fs::read_to_string(input).unwrap();
    let instructions = file.split("\n\n").next().unwrap();
    let nodes: Vec<Node> = file
        .split("\n\n")
        .last()
        .unwrap()
        .lines()
        .map(parse_line)
        .collect();

    let mut node_map: HashMap<String, Children> = Default::default();
    for node in nodes {
        node_map.insert(node.name, Children{
            left: node.left,
            right: node.right,
        });
    }
    let mut curr_node = "AAA";
    let mut steps = 0;
    while curr_node != "ZZZ" {
        let options = node_map.get(curr_node).unwrap();
        match get_step(steps, instructions).unwrap() {
            Step::Left => curr_node = &options.left,
            Step::Right => curr_node = &options.right,
        }
        steps += 1;
    }
    
    steps
}

fn get_step(iter: u32, instructions: &str) -> Option<Step> {
    match instructions.chars().nth(iter as usize % instructions.len()).unwrap() {
        'R' => Some(Step::Right),
        'L' => Some(Step::Left),
        _ => None
    }
}

fn parse_line(line: &str) -> Node {
    let name = line.split(" = ").next().unwrap();
    let raw_children = line.split(" = ").last().unwrap();
    let left = raw_children
        .split(", ")
        .next()
        .unwrap()
        .strip_prefix("(")
        .unwrap();
    let right = &raw_children.split(", ").last().unwrap()[..3];
    Node {
        name: name.to_string(),
        left: left.to_string(),
        right: right.to_string(),
    }
}

#[cfg(test)]
mod test {
    use crate::part1::{parse_line, Node, solve, get_step, Step};

    #[test]
    fn line_parsing() {
        assert_eq!(
            parse_line("AAA = (BBB, CCC)"),
            Node {
                name: "AAA".to_string(),
                left: "BBB".to_string(),
                right: "CCC".to_string(),
            }
        );
        assert_eq!(
            parse_line("BBB = (DDD, EEE)"),
            Node {
                name: "BBB".to_string(),
                left: "DDD".to_string(),
                right: "EEE".to_string(),
            }
        );
    }

    #[test]
    fn instructions() {
        assert_eq!(get_step(0, "RL"), Some(Step::Right));
        assert_eq!(get_step(1, "RL"), Some(Step::Left));
        assert_eq!(get_step(2, "RL"), Some(Step::Right));
        assert_eq!(get_step(3, "RL"), Some(Step::Left));
    }

    #[test]
    fn full() {
        assert_eq!(solve("sample.txt"), 2);
    }
}
