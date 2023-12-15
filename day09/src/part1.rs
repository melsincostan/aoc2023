use std::fs;
pub fn solve(input: &str) -> i64 {
    let seqs: Vec<Vec<i64>> = fs::read_to_string(input).unwrap().lines().map(parse_line).collect();
    seqs.iter().map(next_elem).sum()
}

fn parse_line(line: &str) -> Vec<i64> {
    line.split_whitespace().map(|elem| elem.parse::<i64>().unwrap()).collect()
}

fn next_elem(line: &Vec<i64>) -> i64 {
    let mut lines: Vec<Vec<i64>> = vec![];
    lines.push(line.to_owned());
    while !is_bottom(lines.last().unwrap()) {
        lines.push(sublevel(lines.last().unwrap()));
    }
    lines.reverse();
    lines[0].push(0);
    for i in 0..(lines.len() - 1) {
        let next = lines[i+1].last().unwrap() + lines[i].last().unwrap();
        lines [i+1].push(next);
    }
    *lines.last().unwrap().last().unwrap()
}

fn sublevel(seq: &Vec<i64>) -> Vec<i64> {
    let mut res: Vec<i64> = vec![];
    for i in 0..(seq.len() - 1) {
        res.push(seq[i+1] - seq[i]);
    }
    res
}

fn is_bottom(seq: &Vec<i64>) -> bool {
    seq.iter().all(|num| *num == 0)
}

#[cfg(test)]
mod test {
    use crate::part1::solve;

    #[test]
    fn full() {
        assert_eq!(solve("sample.txt"), 114);
    }
}