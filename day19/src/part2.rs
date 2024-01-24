use std::collections::HashMap;
use std::fs;

#[derive(Debug, PartialEq, Clone)]
struct Rule {
    condition: Option<Condition>,
    action: String,
}

#[derive(Debug, PartialEq, Clone)]
struct Condition {
    tested_value: char,
    operator: char,
    threshold: u64,
}

#[derive(Debug, PartialEq)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

pub fn solve(input: &str) -> u64 {
    let raw_input = fs::read_to_string(input).unwrap();
    let raw_workflows = raw_input
        .split("\n\n")
        .next()
        .unwrap()
        .split("\n")
        .collect::<Vec<&str>>();
    let workflows = parse_workflows(raw_workflows);
    // let first_workflow_name = parse_workflow(raw_input.split("\n\n").next().unwrap().split("\n").next().unwrap()).0;
    let first_workflow_name = "in";
    // do BFS / DFS in the workflows to find all workflows that end in an acceptance
    // problem is representing data in case there are any loops...
    // there doesn't seem to be any!! yay!!
    let selected = search(&workflows, first_workflow_name);
    // now to find all distinct combinations!
    0
}

fn search(workflows: &HashMap<String, Vec<Rule>>, first_workflow_name: &str) -> Vec<Vec<Rule>> {
    // get the first workflow, get all options, see if possible **wrt existing backtrace as well!!!** (no use cluttering up results with stuff that has contradictory conditions)
    let mut selected: Vec<Vec<Rule>> = vec![];
    // add first value to the queue
    let mut queue: Vec<(String, Vec<Rule>)> = vec![];
    queue.push((first_workflow_name.to_string(), vec![]));
    // DFS-ish
    while queue.len() > 0 {
        let (rule_name, backtrace) = queue.pop().unwrap();
        for rule in workflows.get(&rule_name).unwrap() {
            if rule_compatible_with_backtrace(&rule, &backtrace) { // better hope there are no cycles in the graph lol
                let mut new_backtrace = backtrace.to_owned();
                new_backtrace.push(rule.to_owned());
                if rule.action == "A" {
                    selected.push(new_backtrace);
                } else if rule.action != "R" {
                    queue.push((rule.action.to_owned(), new_backtrace));
                }
            }
        }
    }
    println!("Total of {} possible paths", selected.len());
    selected
}

fn rule_compatible_with_backtrace(tested_rule: &Rule, backtrace: &Vec<Rule>) -> bool {
    for rule in backtrace {
        if rule.condition.is_some() && tested_rule.condition.is_some() {
            let condition = rule.condition.to_owned().unwrap();
            let tested_condition = tested_rule.condition.to_owned().unwrap();
            if !conditions_compatible(&condition, &tested_condition) {
                return false;
            }
        }
    }
    true
}

fn conditions_compatible(cond_a: &Condition, cond_b: &Condition) -> bool {
    if cond_a.tested_value != cond_b.tested_value {
        return true;
    } else {
        if cond_a.operator == cond_b.operator {
            return true;
        } else {
            if cond_a.operator == '>' {
                return cond_b.threshold > cond_a.threshold;
            } else {
                return cond_b.threshold < cond_a.threshold;
            }
        }
    }
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
    let raw_rules_list = raw_rules[..raw_rules.len() - 1]
        .split(",")
        .collect::<Vec<&str>>();
    let mut rules: Vec<Rule> = vec![];
    for raw_rule in raw_rules_list {
        rules.push(parse_rule(raw_rule));
    }
    (name.to_string(), rules)
}

fn parse_rule(raw_rule: &str) -> Rule {
    if !raw_rule.contains(">") && !raw_rule.contains(">") && !raw_rule.contains(":") {
        Rule {
            action: raw_rule.to_string(),
            condition: None,
        }
    } else {
        Rule {
            action: raw_rule.split(":").last().unwrap().to_string(),
            condition: Some(parse_condition(raw_rule.split(":").next().unwrap())),
        }
    }
}

fn parse_condition(raw_condition: &str) -> Condition {
    let operator_closure = |c: char| c == '>' || c == '<';
    let tested_value = raw_condition
        .split(operator_closure)
        .next()
        .unwrap()
        .chars()
        .next()
        .unwrap();
    let threshold: u64 = raw_condition
        .split(operator_closure)
        .last()
        .unwrap()
        .parse()
        .unwrap();
    Condition {
        tested_value: tested_value,
        threshold: threshold,
        operator: if raw_condition.contains(">") {
            '>'
        } else {
            '<'
        },
    }
}

#[cfg(test)]
mod test {
    use crate::part2::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 167409079868000);
    }
}
