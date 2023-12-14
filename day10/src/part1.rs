use std::fs;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Node {
    pos: Point,
    children: Vec<Point>,
}

#[derive(PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

pub fn solve(input: &str) -> u32 {
    let input = fs::read_to_string(input).unwrap();
    let mut raw_grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>().to_owned())
        .collect();
    let mut grid: Vec<Vec<Node>> = vec![];
    let grid_size = Point {
        x: raw_grid[0].len() as u32,
        y: raw_grid.len() as u32,
    };
    let mut start_point = Point { x: 0, y: 0 };
    'outer: for y in 0..raw_grid.len() {
        for x in 0..raw_grid[y].len() {
            if raw_grid[y][x] == 'S' {
                start_point = Point {
                    x: x as u32,
                    y: y as u32,
                };
                let east = if x > 0 { raw_grid[y][x - 1] } else { '.' };
                let west = if x < (grid_size.x - 1).try_into().unwrap() {
                    raw_grid[y][x + 1]
                } else {
                    '.'
                };
                let north = if y > 0 { raw_grid[y - 1][x] } else { '.' };
                let south = if y < (grid_size.y - 1).try_into().unwrap() {
                    raw_grid[y + 1][x]
                } else {
                    '.'
                };
                raw_grid[y][x] = start_type(east, west, north, south).unwrap();
                break 'outer;
            }
        }
    }

    for (i, line) in raw_grid.iter().enumerate() {
        grid.push(parse_line(line, i, &grid_size));
    }

    let mut curr_pos = grid[start_point.y as usize][start_point.x as usize].pos;
    let mut visited: Vec<Point> = vec![start_point];
    'outer: loop {
        for child in &grid[curr_pos.y as usize][curr_pos.x as usize].children {
            if !visited.contains(child) {
                curr_pos = *child;
                visited.push(*child);
                break;
            }
            if *child == start_point && visited.len() > 1 {
                break 'outer;
            }
        }
    }
    (visited.len() / 2).try_into().unwrap()
}

fn start_type(east: char, west: char, north: char, south: char) -> Option<char> {
    if connects(Direction::North, north) && connects(Direction::South, south) {
        Some('|')
    } else if connects(Direction::East, east) && connects(Direction::West, west) {
        Some('-')
    } else if connects(Direction::North, north) && connects(Direction::East, east) {
        Some('J')
    } else if connects(Direction::North, north) && connects(Direction::West, west) {
        Some('L')
    } else if connects(Direction::South, south) && connects(Direction::West, west) {
        Some('F')
    } else if connects(Direction::South, south) && connects(Direction::East, east) {
        Some('7')
    } else {
        None
    }
}

fn connects(pipe_direction: Direction, pipe_type: char) -> bool {
    if pipe_direction == Direction::North {
        if pipe_type == '|' || pipe_type == '7' || pipe_type == 'F' {
            true
        } else {
            false
        }
    } else if pipe_direction == Direction::South {
        if pipe_type == '|' || pipe_type == 'L' || pipe_type == 'J' {
            true
        } else {
            false
        }
    } else if pipe_direction == Direction::East {
        if pipe_type == '-' || pipe_type == 'L' || pipe_type == 'F' {
            true
        } else {
            false
        }
    } else {
        // implicit Direction::West
        if pipe_type == '-' || pipe_type == '7' || pipe_type == 'J' {
            true
        } else {
            false
        }
    }
}

fn parse_line(line: &Vec<char>, y: usize, grid_size: &Point) -> Vec<Node> {
    let mut parsed_line: Vec<Node> = vec![];
    for (i, c) in line.iter().enumerate() {
        parsed_line.push(parse_char(&c, i, y, grid_size))
    }
    parsed_line
}

fn parse_char(c: &char, x: usize, y: usize, grid_size: &Point) -> Node {
    match c {
        '.' => Node {
            pos: Point {
                x: x as u32,
                y: y as u32,
            },
            children: vec![],
        },
        _ => Node {
            pos: Point {
                x: x as u32,
                y: y as u32,
            },
            children: get_children(c, Point{
                x: x as u32,
                y: y as u32,
            }, grid_size),
        }
    }
}

fn get_children(node_type: &char, pos: Point, grid_size: &Point) -> Vec<Point> {
    let mut children: Vec<Point> = vec![];
    match node_type {
        '-' => {
            if pos.x > 0 {
                children.push(make_point(pos.x - 1, pos.y));
            }
            if pos.x < grid_size.x - 1 {
                children.push(make_point(pos.x + 1, pos.y));
            }
        }
        '|' => {
            if pos.y > 0 {
                children.push(make_point(pos.x, pos.y - 1));
            }
            if pos.y < grid_size.y - 1 {
                children.push(make_point(pos.x, pos.y + 1));
            }
        }
        'L' => {
            if pos.x < grid_size.x - 1 {
                children.push(make_point(pos.x + 1, pos.y));
            }
            if pos.y > 0 {
                children.push(make_point(pos.x, pos.y - 1));
            }
        }
        'J' => {
            if pos.x > 0 {
                children.push(make_point(pos.x - 1, pos.y));
            }
            if pos.y > 0 {
                children.push(make_point(pos.x, pos.y - 1));
            }
        }
        '7' => {
            if pos.x > 0 {
                children.push(make_point(pos.x - 1, pos.y));
            }
            if pos.y < grid_size.y - 1 {
                children.push(make_point(pos.x, pos.y + 1));
            }
        }
        'F' => {
            if pos.x < grid_size.x - 1 {
                children.push(make_point(pos.x + 1, pos.y));
            }
            if pos.y < grid_size.y - 1 {
                children.push(make_point(pos.x, pos.y + 1));
            }
        }
        _ => {}
    }
    children
}

fn make_point(x: u32, y: u32) -> Point {
    // makes the syntax for creating points shorter :S
    Point { x: x, y: y }
}
