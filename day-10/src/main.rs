use itertools::Itertools;
use std::io::BufRead;
use std::{fs::File, io::BufReader};

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    All,
}

impl Direction {
    fn pos_to_dir(&self, x: &i32, y: &i32) -> (i32, i32) {
        match self {
            Direction::Left => (x - 1, *y),
            Direction::Right => (x + 1, *y),
            Direction::Down => (*x, y + 1),
            Direction::Up => (*x, y - 1),
            _ => unreachable!(),
        }
    }

    fn dir_to_pos(x: &i32, y: &i32) -> Direction {
        match (x, y) {
            (0, 1) => Direction::Down,
            (0, -1) => Direction::Up,
            (1, 0) => Direction::Right,
            (-1, 0) => Direction::Left,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Pipe {
    is_start: bool,
    pos: (i32, i32),
    in_out: Option<(Direction, Direction)>,
}

impl Pipe {
    fn reachable_from(&self, from_x: &i32, from_y: &i32) -> bool {
        self.pass_by(from_x, from_y).is_some()
    }

    fn pass_by(&self, from_x: &i32, from_y: &i32) -> Option<(i32, i32)> {
        // from above
        if *from_x == self.pos.0 && *from_y == self.pos.1 + 1 {
            match self.in_out {
                Some((Direction::Down, ref dir)) => {
                    return Some(dir.pos_to_dir(&self.pos.0, &self.pos.1))
                }
                Some((ref dir, Direction::Down)) => {
                    return Some(dir.pos_to_dir(&self.pos.0, &self.pos.1))
                }
                _ => return None,
            }
        }

        // from below
        if *from_x == self.pos.0 && *from_y == self.pos.1 - 1 {
            match self.in_out {
                Some((Direction::Up, ref dir)) => {
                    return Some(dir.pos_to_dir(&self.pos.0, &self.pos.1))
                }
                Some((ref dir, Direction::Up)) => {
                    return Some(dir.pos_to_dir(&self.pos.0, &self.pos.1))
                }
                _ => return None,
            }
        }

        // from left
        if *from_x == self.pos.0 - 1 && *from_y == self.pos.1 {
            match self.in_out {
                Some((Direction::Left, ref dir)) => {
                    return Some(dir.pos_to_dir(&self.pos.0, &self.pos.1))
                }
                Some((ref dir, Direction::Left)) => {
                    return Some(dir.pos_to_dir(&self.pos.0, &self.pos.1))
                }
                _ => return None,
            }
        }

        // from right
        if *from_x == self.pos.0 + 1 && *from_y == self.pos.1 {
            match self.in_out {
                Some((Direction::Right, ref dir)) => {
                    return Some(dir.pos_to_dir(&self.pos.0, &self.pos.1))
                }
                Some((ref dir, Direction::Right)) => {
                    return Some(dir.pos_to_dir(&self.pos.0, &self.pos.1))
                }
                _ => return None,
            }
        }
        None
    }
}

fn main() -> std::io::Result<()> {
    let mut grid: Vec<Vec<Pipe>> = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|line| line.ok())
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '|' => Pipe {
                        is_start: false,
                        pos: (x as i32, y as i32),
                        in_out: Some((Direction::Up, Direction::Down)),
                    },
                    '-' => Pipe {
                        is_start: false,
                        pos: (x as i32, y as i32),
                        in_out: Some((Direction::Left, Direction::Right)),
                    },
                    'L' => Pipe {
                        is_start: false,
                        pos: (x as i32, y as i32),
                        in_out: Some((Direction::Up, Direction::Right)),
                    },
                    'J' => Pipe {
                        is_start: false,
                        pos: (x as i32, y as i32),
                        in_out: Some((Direction::Up, Direction::Left)),
                    },
                    '7' => Pipe {
                        is_start: false,
                        pos: (x as i32, y as i32),
                        in_out: Some((Direction::Down, Direction::Left)),
                    },
                    'F' => Pipe {
                        is_start: false,
                        pos: (x as i32, y as i32),
                        in_out: Some((Direction::Down, Direction::Right)),
                    },
                    '.' => Pipe {
                        is_start: false,
                        pos: (x as i32, y as i32),
                        in_out: None,
                    },
                    'S' => Pipe {
                        is_start: true,
                        pos: (x as i32, y as i32),
                        in_out: Some((Direction::Up, Direction::Down)),
                    },
                    _ => panic!(),
                })
                .collect_vec()
        })
        .collect_vec();

    let (start_x, start_y) = grid
        .iter()
        .enumerate()
        .find_map(
            |(y, line)| match line.iter().enumerate().find(|(_x, c)| c.is_start) {
                Some((x, _)) => Some((x, y)),
                None => None,
            },
        )
        .map(|(x, y)| (x as i32, y as i32))
        .unwrap();

    println!("starts is at: {start_x:?} {start_y:?}");

    // left, right, down, up
    let niegh: [(i32, i32); 4] = [(0, -1), (0, 1), (1, 0), (-1, 0)];

    let directions = niegh
        .iter()
        .filter(|(n_x, n_y)| {
            let current = (start_x + n_x, start_y + n_y);

            if current.0 >= 0
                && current.1 >= 0
                && current.1 < grid.len() as i32
                && current.0 < grid[0].len() as i32
            {
                let try_to_go = grid[current.1 as usize][current.0 as usize];
                return try_to_go.reachable_from(&start_x, &start_y);
            } else {
                return false;
            }
        })
        .map(|(x, y)| Direction::dir_to_pos(x, y))
        .collect_tuple();

    let start_pipe = Pipe {
        pos: (start_x, start_y),
        in_out: directions,
        is_start: true,
    };
    grid[start_y as usize][start_x as usize] = start_pipe;

    let current = (start_x, start_y);
    let next = match start_pipe.in_out {
        Some((dir, _)) => dir.pos_to_dir(&start_x, &start_y),
        None => panic!(),
    };

    let mut optional = Some((current, next));

    let mut loop_size = 0;
    while let Some(((x, y), next)) = optional {
        // println!("{:?} -> {:?}", (x, y), next);
        if x == start_x && y == start_y && loop_size > 0 {
            break;
        }
        loop_size += 1;
        let pipe = grid[next.1 as usize][next.0 as usize];
        let next_next = pipe.pass_by(&x, &y).unwrap();
        optional = Some((next, next_next));
    }

    let farthest_from_start = loop_size / 2;
    println!("p1: {farthest_from_start:?}");

    Ok(())
}
