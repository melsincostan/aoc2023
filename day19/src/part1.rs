use std::fs;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Rule {
    condition: Option<Condition>,
    action: String,
}

#[derive(Debug, PartialEq)]
struct Condition {
    tested_value: char,
    operator: char,
    threshold: u32,
}

#[derive(Debug, PartialEq)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

pub fn solve(input: &str) -> u32 {
    let raw_input = fs::read_to_string(input).unwrap();
    let raw_workflows = raw_input.split("\n\n").next().unwrap().split("\n").collect::<Vec<&str>>();
    let workflows = parse_workflows(raw_workflows);
    // let first_workflow_name = parse_workflow(raw_input.split("\n\n").next().unwrap().split("\n").next().unwrap()).0;
    let first_workflow_name = "in";
    let parts = raw_input.split("\n\n").last().unwrap().split("\n").map(|p| parse_part(p)).collect::<Vec<Part>>();
    let mut accepted: Vec<Part> = vec![];
    for part in parts {
        if process_part(&part, &workflows, &first_workflow_name) {
            accepted.push(part);
        }
    }
    accepted.iter().map(|p| sum_part(p)).sum()
}

fn process_part(part: &Part, workflows: &HashMap<String, Vec<Rule>>, curr_workflow: &str) -> bool {
    if !workflows.contains_key(curr_workflow) {
        panic!("Unknown workflow: {}", curr_workflow);
    } else {
        let rules = workflows.get(curr_workflow).unwrap();
        for rule in rules {
            if rule.condition.is_some() && !part_matches_condition(part, &rule.condition.as_ref().unwrap()){
                continue;
            }
            match rule.action.as_str() {
                "A" => return true,
                "R" => return false,
                _ => return process_part(part, workflows, &rule.action)
            };
        }
    }
    panic!("Got part with no proper resolution: {:?}", part);
}

fn part_matches_condition(part: &Part, condition: &Condition) -> bool {
    let testval;
    match condition.tested_value {
        'x' => testval = part.x,
        'm' => testval = part.m,
        'a' => testval = part.a,
        's' => testval = part.s,
        _ => panic!("Unexpected value to test for: {}", condition.tested_value)
    };
    match condition.operator {
        '>' => testval > condition.threshold,
        '<' => testval < condition.threshold,
        _ => panic!("Unexpected operator: {}", condition.operator)
    }
}

fn sum_part(part: &Part) -> u32 {
    part.x + part.m + part.a + part.s
}

fn parse_part(raw_part: &str) -> Part {
    let values = raw_part[1..raw_part.len() - 1].split(",").collect::<Vec<&str>>();
    let mut part = Part{
        x: 0,
        m: 0,
        a: 0,
        s: 0,
    };
    for val in values {
        let letter = val.split("=").next().unwrap();
        let value: u32 = val.split("=").last().unwrap().parse().unwrap();
        match letter {
            "x" => part.x = value,
            "m" => part.m = value,
            "a" => part.a = value,
            "s" => part.s = value,
            _ => panic!("Unknown value type \"{}\"", letter)
        }
    }
    part
}

fn parse_workflows(raw_flows: Vec<&str>) -> HashMap<String, Vec<Rule>> {
    let mut flows: HashMap<String, Vec<Rule>> = HashMap::new();
    for raw_flow in raw_flows {
        let parsed_flow = parse_workflow(raw_flow);
        flows.insert(parsed_flow.0, parsed_flow.1);
    }
    flows
}

fn parse_workflow(raw_flow: &str) -> (String, Vec<Rule>) {
    let name = raw_flow.split("{").next().unwrap();
    let raw_rules = raw_flow.split("{").last().unwrap();
    let raw_rules_list = raw_rules[..raw_rules.len() - 1].split(",").collect::<Vec<&str>>();
    let mut rules: Vec<Rule> = vec![];
    for raw_rule in raw_rules_list {
        rules.push(parse_rule(raw_rule));
    }
    (name.to_string(), rules)
}

fn parse_rule(raw_rule: &str) -> Rule {
    if !raw_rule.contains(">") && !raw_rule.contains(">") && !raw_rule.contains(":") {
        Rule{
            action: raw_rule.to_string(),
            condition: None,
        }
    } else {
        Rule{
            action: raw_rule.split(":").last().unwrap().to_string(),
            condition: Some(parse_condition(raw_rule.split(":").next().unwrap()))
        }
    }
}

fn parse_condition(raw_condition: &str) -> Condition {
    let operator_closure = |c: char| {c == '>' || c == '<'};
    let tested_value = raw_condition.split(operator_closure).next().unwrap().chars().next().unwrap();
    let threshold: u32 = raw_condition.split(operator_closure).last().unwrap().parse().unwrap();
    Condition{
        tested_value: tested_value,
        threshold: threshold,
        operator: if raw_condition.contains(">") {'>'} else {'<'},
    }
}

#[cfg(test)]
mod test {
    use crate::part1::{solve, parse_rule, Rule, Condition, parse_condition, parse_workflow, parse_part, Part, sum_part};

    #[test]
    fn test_sum_part() {
        let part = parse_part("{x=787,m=2655,a=1222,s=2876}");
        assert_eq!(sum_part(&part), 7540);
    }

    #[test]
    fn test_parse_part() {
        assert_eq!(parse_part("{x=787,m=2655,a=1222,s=2876}"), Part{
            x: 787,
            m: 2655,
            a: 1222,
            s: 2876,
        });
    }

    #[test]
    fn test_parse_workflow() {
        assert_eq!(parse_workflow("px{a<2006:qkq,m>2090:A,rfg}").0, "px".to_string());
        assert_eq!(parse_workflow("px{a<2006:qkq,m>2090:A,rfg}").1.len(), 3);
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 19114);
    }

    #[test]
    fn test_parse_rule() {
        assert_eq!(parse_rule("x>2662:A"), Rule{
            action: "A".to_string(),
            condition: Some(Condition{
                tested_value: 'x',
                operator: '>',
                threshold: 2662,
            })
        });
        assert_eq!(parse_rule("pv"), Rule{
            action: "pv".to_string(),
            condition: None,
        });
    }

    #[test]
    fn test_parse_condition() {
        assert_eq!(parse_condition("a<2006"), Condition{
            operator: '<',
            tested_value: 'a',
            threshold: 2006,
        });
        assert_eq!(parse_condition("a>1716"), Condition{
            operator: '>',
            tested_value: 'a',
            threshold: 1716,
        });
    }
}