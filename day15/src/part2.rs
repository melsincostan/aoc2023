use std::fs;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Lens {
    label: String,
    focusing_power: u32,
}

pub fn solve(input: &str) -> u32 {
    let raw_input = fs::read_to_string(input).unwrap();
    let steps: Vec<&str> = raw_input.trim().split(",").collect();
    let mut buckets: Vec<Vec<Lens>> = vec![vec![]; 256]; // hash map buckets

    for step in steps {
        let instructions = parse_step(step);
        let label = instructions.0;
        let bucket = hash(&label);
        let operation = instructions.1;
        if operation == '=' {
            let new_lens = Lens{
                label: label,
                focusing_power: instructions.2.unwrap()
            };
            let mut new_contents: Vec<Lens> = vec![];
            let mut pushed_new_lens = false;
            for lens in &buckets[bucket] {
                if lens.label != new_lens.label {
                    new_contents.push(lens.to_owned());
                } else {
                    new_contents.push(new_lens.clone());
                    pushed_new_lens = true;
                }
            }
            if !pushed_new_lens {
                new_contents.push(new_lens.clone());
            }
            buckets[bucket] = new_contents;
        } else if operation == '-' {
            let mut new_contents: Vec<Lens> = vec![];
            for lens in &buckets[bucket] {
                if lens.label != label {
                    new_contents.push(lens.to_owned());
                }
            }
            buckets[bucket] = new_contents;
        }
    }

    let mut total_focusing_power = 0;
    for (i, bucket) in buckets.iter().enumerate() {
        for (j, lens) in bucket.iter().enumerate() {
            total_focusing_power += (i as u32 +1) * (j as u32+1) * lens.focusing_power;
        }
    }
    total_focusing_power
}

fn parse_step(step: &str) -> (String, char, Option<u32>) {
    if step.ends_with("-") {
        (step[..step.len() - 1].to_string(), '-', None)
    } else {
        let mut split_step = step.split("=");
        (split_step.next().unwrap().to_string(), '=', Some(split_step.last().unwrap().parse::<u32>().unwrap()))
    }
}

fn hash(string: &str) -> usize {
    let mut hash_result = 0;
    for c in string.chars() {
        hash_result += c as u32;
        hash_result *= 17;
        hash_result %= 256;
    }
    hash_result as usize
}

#[cfg(test)]
mod test {
    use crate::part2::{solve, parse_step};

    #[test]
    fn test_parse_step() {
        assert_eq!(parse_step("rn=1"), ("rn".to_string(), '=', Some(1)));
        assert_eq!(parse_step("cm-"), ("cm".to_string(), '-', None));
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 145);
    }
}
