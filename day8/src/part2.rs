use std::fs;
use std::collections::HashMap;
use num::integer::lcm;

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

pub fn solve(input: &str) -> u64 {
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
    let mut curr_nodes: Vec<&String> = vec![];
    for node in nodes {
        node_map.insert(node.name, Children{
            left: node.left,
            right: node.right,
        });
    }

    for node in node_map.keys() {
        if node.ends_with("A") {
            curr_nodes.push(node);
        } 
    }

    let mut cycles: Vec<u64> = vec![];
    for node in curr_nodes {
        let mut curr_node = node;
        let mut steps = 0;
        loop {
            if curr_node.ends_with("Z") {
                cycles.push(steps);
                break;
            }
            let options = node_map.get(curr_node).unwrap();
            match get_step(steps, instructions).unwrap() {
                Step::Left => curr_node = &options.left,
                Step::Right => curr_node = &options.right,
            }
            steps += 1;
        }
    }

    arr_lcm(&cycles)
}

fn arr_lcm(arr: &Vec<u64>) -> u64 {
    recur_arr_lcm(arr[0], arr[1..].to_owned())
}

fn recur_arr_lcm(n: u64, arr: Vec<u64>) -> u64{
    if arr.len() == 1 {
        return lcm(n, arr[0]);
    }
    recur_arr_lcm(lcm(n, arr[0]), arr[1..].to_owned())
}

fn get_step(iter: u64, instructions: &str) -> Option<Step> {
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
    use crate::part2::solve;

    #[test]
    fn full() {
        assert_eq!(solve("sample2.txt"), 6);
    }
}
