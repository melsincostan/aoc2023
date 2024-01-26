use std::{collections::HashSet, fs};

pub fn solve(input: &str, steps: u32) -> u32 {
    let map = fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let width = map[0].len();
    let height = map.len();
    let mut startpos: (usize, usize) = (0, 0);
    'outer: for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 'S' {
                startpos = (x, y);
                break 'outer;
            }
        }
    }

    let mut queue: Vec<(u32, (usize, usize))> = vec![]; // queue for DFS-ish, doesn't really matter what one does here i think?
    let mut reachable_positions: HashSet<(usize, usize)> = HashSet::default(); // positions reachable after `steps` steps, deduped
    queue.push((0, startpos));

    while queue.len() > 0 {
        let (steps_so_far, current_position) = queue.pop().unwrap();
        for neighbor in neighbors(current_position, width, height) {
            match map[neighbor.1][neighbor.0] {
                '.' | 'S' => {
                    if steps_so_far == steps - 1 {
                        reachable_positions.insert(neighbor);
                    } else {
                        queue.push((steps_so_far + 1, neighbor));
                    }
                },
                _ => {
                    continue;
                }
            }
        }
    }

    reachable_positions.len() as u32
}

fn neighbors(pos: (usize, usize), w: usize, h: usize) -> Vec<(usize, usize)> {
    let (x, y) = pos;
    let mut res: Vec<(usize, usize)> = vec![];
    if x > 0 {
        res.push((x - 1, y));
    }
    if x < w - 1 {
        res.push((x + 1, y));
    }
    if y > 0 {
        res.push((x, y - 1));
    }
    if y < h - 1 {
        res.push((x, y + 1));
    }
    res
}

#[cfg(test)]
mod test {
    use crate::part1::{neighbors, solve};

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt", 6), 16);
    }

    #[test]
    fn test_neighbors() {
        assert_eq!(neighbors((1, 1), 3, 3).len(), 4); // middle of a 3x3 grid: all moves available
        assert_eq!(neighbors((0, 0), 3, 3).len(), 2); // top left corner of a 3x3 grid: only down and right move available
        assert_eq!(neighbors((0, 1), 3, 3).len(), 3); // middle position on the top row of a 3x3 grid: only up move isn't available
    }
}
