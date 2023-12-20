use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
    direction: Direction,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn solve(input: &str) -> u32 {
    let input = fs::read_to_string(input).unwrap();
    let max_y = input.lines().count();
    let max_x = input.lines().next().unwrap().chars().count();
    let mut starting_positions: Vec<Point> = vec![];

    for x in 0..max_x {
        starting_positions.push(Point{
            x: x as i32,
            y: 0,
            direction: Direction::Down
        });
        starting_positions.push(Point{
            x: x as i32,
            y: (max_y - 1) as i32,
            direction: Direction::Up,
        });
    }
    for y in 0..max_y {
        starting_positions.push(Point{
            x: 0,
            y: y as i32,
            direction: Direction::Right,
        });
        starting_positions.push(Point{
            x: (max_x - 1) as i32,
            y: y as i32,
            direction: Direction::Left,
        });
    }

    starting_positions.iter().map(|pos| get_energy(&input, pos.to_owned())).max().unwrap()
}

fn get_energy(input: &str, start_point: Point) -> u32 {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let grid_width = grid[0].len();
    let grid_height = grid.len();
    let mut queue: VecDeque<Point> = Default::default();
    let mut visited: HashSet<Point> = Default::default();
    let mut energized: HashSet<(usize, usize)> = Default::default();

    queue.push_back(start_point);

    loop {
        if queue.len() < 1 {
            break;
        }
        let point_to_visit = queue.pop_front().unwrap();
        visited.insert(point_to_visit);
        energized.insert((point_to_visit.x as usize, point_to_visit.y as usize));
        let point_type = grid[point_to_visit.y as usize][point_to_visit.x as usize];
        match point_type {
            '.' => {
                let next_point = next_point_dot(&point_to_visit, grid_height, grid_width);
                if next_point.is_some() && !visited.contains(&next_point.unwrap()) {
                    queue.push_back(next_point.unwrap());
                }
            }
            '/' | '\\' => {
                let next_point =
                    next_point_mirror(&point_to_visit, point_type, grid_height, grid_width);
                if next_point.is_some() && !visited.contains(&next_point.unwrap()) {
                    queue.push_back(next_point.unwrap());
                }
            }
            '|' | '-' => {
                let next_points =
                    next_point_splitter(&point_to_visit, point_type, grid_height, grid_width);
                for next_point in next_points {
                    if next_point.is_some() && !visited.contains(&next_point.unwrap()) {
                        queue.push_back(next_point.unwrap());
                    }
                }
            }
            _ => panic!("Unexpected point type: {}", point_type),
        }
    }

    energized.len() as u32
}

fn next_point_splitter(
    current_point: &Point,
    splitter: char,
    grid_height: usize,
    grid_width: usize,
) -> Vec<Option<Point>> {
    let mut points: Vec<Option<Point>> = vec![];
    let mut point_a = current_point.clone();
    let mut point_b = current_point.clone();

    if splitter == '|' {
        match current_point.direction {
            Direction::Down | Direction::Up => {
                points.push(next_point_dot(&point_a, grid_height, grid_width))
            }
            Direction::Left | Direction::Right => {
                point_a.direction = Direction::Up;
                point_b.direction = Direction::Down;
                points.push(next_point_dot(&point_a, grid_height, grid_width));
                points.push(next_point_dot(&point_b, grid_height, grid_width));
            }
        }
    } else if splitter == '-' {
        match current_point.direction {
            Direction::Left | Direction::Right => {
                points.push(next_point_dot(&point_a, grid_height, grid_width))
            }
            Direction::Down | Direction::Up => {
                point_a.direction = Direction::Left;
                point_b.direction = Direction::Right;
                points.push(next_point_dot(&point_a, grid_height, grid_width));
                points.push(next_point_dot(&point_b, grid_height, grid_width));
            }
        }
    }
    points
}

fn next_point_mirror(
    current_point: &Point,
    mirror: char,
    grid_height: usize,
    grid_width: usize,
) -> Option<Point> {
    let mut point = current_point.clone();
    if mirror == '/' {
        match current_point.direction {
            Direction::Right => point.direction = Direction::Up,
            Direction::Left => point.direction = Direction::Down,
            Direction::Up => point.direction = Direction::Right,
            Direction::Down => point.direction = Direction::Left,
        }
    } else if mirror == '\\' {
        // it will never be anything else, but this makes it clearer
        match current_point.direction {
            Direction::Right => point.direction = Direction::Down,
            Direction::Left => point.direction = Direction::Up,
            Direction::Up => point.direction = Direction::Left,
            Direction::Down => point.direction = Direction::Right,
        }
    }
    next_point_dot(&point, grid_height, grid_width)
}

fn next_point_dot(current_point: &Point, grid_height: usize, grid_width: usize) -> Option<Point> {
    let mut point = current_point.clone();
    match current_point.direction {
        Direction::Right => point.x += 1,
        Direction::Left => point.x -= 1,
        Direction::Up => point.y -= 1,
        Direction::Down => point.y += 1,
    }
    if point_valid(&point, grid_height, grid_width) {
        Some(point)
    } else {
        None
    }
}

fn point_valid(point: &Point, grid_height: usize, grid_width: usize) -> bool {
    point.x >= 0 && point.x < (grid_width as i32) && point.y >= 0 && point.y < (grid_height as i32)
}

#[cfg(test)]
mod test {
    use crate::part2::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 51);
    }
}
