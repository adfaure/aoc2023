use itertools::Itertools;
use std::collections::HashSet;
use std::io::BufRead;
use std::{fs::File, io::BufReader};

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    UpRigh,
    UpLeft,
    DownRight,
    DownLeft,
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

    fn inverse(&self) -> Direction {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::DownLeft => Direction::UpRigh,
            Direction::DownRight => Direction::UpLeft,
            Direction::UpLeft => Direction::DownRight,
            Direction::UpRigh => Direction::DownLeft,
            _ => panic!(),
        }
    }

    fn get_neighbors(&self, x: &i32, y: &i32) -> (Option<(i32, i32)>, Option<(i32, i32)>) {
        let rs = match self {
            Direction::UpRigh => Some((
                Direction::Up.get_neighbors(x, y).0,
                Direction::Right.get_neighbors(x, y).0,
            )),
            Direction::UpLeft => Some((
                Direction::Up.get_neighbors(x, y).0,
                Direction::Left.get_neighbors(x, y).0,
            )),
            Direction::DownRight => Some((
                Direction::Down.get_neighbors(x, y).0,
                Direction::Right.get_neighbors(x, y).0,
            )),
            Direction::DownLeft => Some((
                Direction::Down.get_neighbors(x, y).0,
                Direction::Left.get_neighbors(x, y).0,
            )),
            _ => None,
        };

        if rs.is_some() {
            return rs.unwrap();
        }

        let new_dir = match self {
            Direction::Left => (x - 1, *y),
            Direction::Right => (x + 1, *y),
            Direction::Down => (*x, y + 1),
            Direction::Up => (*x, y - 1),
            _ => unreachable!(),
        };
        return (Some(new_dir), None);
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

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct Pipe {
    is_main_loop: bool,
    pos: (i32, i32),
    c: char,
    in_out: Option<(Direction, Direction)>,
}

impl Pipe {
    fn reachable_from(&self, from_x: &i32, from_y: &i32) -> bool {
        self.pass_by(from_x, from_y).is_some()
    }

    fn after(&self) -> Option<(i32, i32)> {
        match self.in_out {
            Some((_b, a)) => Some(a.pos_to_dir(&self.pos.0, &self.pos.1)),
            None => None,
        }
    }

    fn before(&self) -> Option<(i32, i32)> {
        match self.in_out {
            Some((b, _)) => Some(b.pos_to_dir(&self.pos.0, &self.pos.1)),
            None => None,
        }
    }

    fn follow_outside(&self, dir: &Direction) -> Option<Direction> {
        let dir = *dir;
        match self.c {
            '-' => None,
            '|' => None,

            // L
            'L' => {
                if dir == Direction::Down {
                    Some(Direction::Left)
                } else if dir == Direction::Left {
                    Some(Direction::Down)
                } else if dir == Direction::Right {
                    Some(Direction::Up)
                } else if dir == Direction::Up {
                    Some(Direction::Right)
                } else {
                    unreachable!();
                }
            }
            // J
            'J' => {
                if dir == Direction::Down {
                    Some(Direction::Right)
                } else if dir == Direction::Left {
                    Some(Direction::Up)
                } else if dir == Direction::Right {
                    Some(Direction::Down)
                } else if dir == Direction::Up {
                    Some(Direction::Left)
                } else {
                    unreachable!();
                }
            }
            // 7
            '7' => {
                if dir == Direction::Down {
                    Some(Direction::Left)
                } else if dir == Direction::Left {
                    Some(Direction::Down)
                } else if dir == Direction::Right {
                    Some(Direction::Up)
                } else if dir == Direction::Up {
                    Some(Direction::Right)
                } else {
                    unreachable!();
                }
            }
            // F
            'F' => {
                if dir == Direction::Down {
                    Some(Direction::Right)
                } else if dir == Direction::Left {
                    Some(Direction::Up)
                } else if dir == Direction::Right {
                    Some(Direction::Down)
                } else if dir == Direction::Up {
                    Some(Direction::Left)
                } else {
                    unreachable!();
                }
            }
            _ => panic!(),
        }
    }

    fn pass_by(&self, from_x: &i32, from_y: &i32) -> Option<((i32, i32), Direction)> {
        // from above
        if *from_x == self.pos.0 && *from_y == self.pos.1 + 1 {
            match self.in_out {
                Some((Direction::Down, ref dir)) => {
                    return Some((dir.pos_to_dir(&self.pos.0, &self.pos.1), *dir))
                }
                Some((ref dir, Direction::Down)) => {
                    return Some((dir.pos_to_dir(&self.pos.0, &self.pos.1), *dir))
                }
                _ => return None,
            }
        }

        // from below
        if *from_x == self.pos.0 && *from_y == self.pos.1 - 1 {
            match self.in_out {
                Some((Direction::Up, ref dir)) => {
                    return Some((dir.pos_to_dir(&self.pos.0, &self.pos.1), *dir))
                }
                Some((ref dir, Direction::Up)) => {
                    return Some((dir.pos_to_dir(&self.pos.0, &self.pos.1), *dir))
                }
                _ => return None,
            }
        }

        // from left
        if *from_x == self.pos.0 - 1 && *from_y == self.pos.1 {
            match self.in_out {
                Some((Direction::Left, ref dir)) => {
                    return Some((dir.pos_to_dir(&self.pos.0, &self.pos.1), *dir))
                }
                Some((ref dir, Direction::Left)) => {
                    return Some((dir.pos_to_dir(&self.pos.0, &self.pos.1), *dir))
                }
                _ => return None,
            }
        }

        // from right
        if *from_x == self.pos.0 + 1 && *from_y == self.pos.1 {
            match self.in_out {
                Some((Direction::Right, ref dir)) => {
                    return Some((dir.pos_to_dir(&self.pos.0, &self.pos.1), *dir))
                }
                Some((ref dir, Direction::Right)) => {
                    return Some((dir.pos_to_dir(&self.pos.0, &self.pos.1), *dir))
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
                        is_main_loop: false,
                        pos: (x as i32, y as i32),
                        c,
                        in_out: Some((Direction::Up, Direction::Down)),
                    },
                    '-' => Pipe {
                        is_main_loop: false,
                        c,
                        pos: (x as i32, y as i32),
                        in_out: Some((Direction::Left, Direction::Right)),
                    },
                    'L' => Pipe {
                        is_main_loop: false,
                        c: c,
                        pos: (x as i32, y as i32),
                        in_out: Some((Direction::Up, Direction::Right)),
                    },
                    'J' => Pipe {
                        is_main_loop: false,
                        c: c,
                        pos: (x as i32, y as i32),
                        in_out: Some((Direction::Up, Direction::Left)),
                    },
                    '7' => Pipe {
                        is_main_loop: false,
                        pos: (x as i32, y as i32),
                        c: c,
                        in_out: Some((Direction::Down, Direction::Left)),
                    },
                    'F' => Pipe {
                        is_main_loop: false,
                        pos: (x as i32, y as i32),
                        c: c,
                        in_out: Some((Direction::Down, Direction::Right)),
                    },
                    '.' => Pipe {
                        is_main_loop: false,
                        c: c,
                        pos: (x as i32, y as i32),
                        in_out: None,
                    },
                    'S' => Pipe {
                        is_main_loop: true,
                        pos: (x as i32, y as i32),
                        c: c,
                        in_out: None,
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
            |(y, line)| match line.iter().enumerate().find(|(_x, c)| c.is_main_loop) {
                Some((x, _)) => Some((x, y)),
                None => None,
            },
        )
        .map(|(x, y)| (x as i32, y as i32))
        .unwrap();

    println!("starts is at: {start_x:?} {start_y:?}");

    // left, right, down, up
    let niegh: [(i32, i32); 4] = [(1, 0), (0, -1), (0, 1), (-1, 0)];

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
        is_main_loop: true,
        c: match directions {
            Some((Direction::Right, Direction::Left))
            | Some((Direction::Left, Direction::Right)) => '-',
            Some((Direction::Up, Direction::Down)) | Some((Direction::Down, Direction::Up)) => '|',
            Some((Direction::Up, Direction::Left)) | Some((Direction::Left, Direction::Up)) => 'J',
            Some((Direction::Up, Direction::Right)) | Some((Direction::Right, Direction::Up)) => {
                'L'
            }
            Some((Direction::Down, Direction::Right))
            | Some((Direction::Right, Direction::Down)) => 'F',
            Some((Direction::Down, Direction::Left)) | Some((Direction::Left, Direction::Down)) => {
                '7'
            }
            _ => {
                panic!();
            }
        },
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
        if x == start_x && y == start_y && loop_size > 0 {
            break;
        }
        loop_size += 1;
        let pipe = &mut grid[next.1 as usize][next.0 as usize];
        pipe.is_main_loop = true;

        let next_next = pipe.pass_by(&x, &y).unwrap();
        optional = Some((next, next_next.0));
    }

    let farthest_from_start = loop_size / 2;
    println!("p1: {farthest_from_start:?}");

    let mut seen = HashSet::new();
    let mut seen_pipe_side = HashSet::new();
    let mut total_enclosed = 0;

    let (x, y) = (14, 3);
    // yeah try all
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if !seen.contains(&(x as i32, y as i32)) && !grid[y as usize][x as usize].is_main_loop {
                // println!("currently at pos {:?}", (x, y));

                let mut stack = Vec::new();
                stack.push((x as i32, y as i32));

                let mut enclosed = true;

                let mut current_batch = HashSet::new();
                current_batch.insert((x as i32, y as i32));

                while let Some((x, y)) = stack.pop() {
                    // println!("popping: {:?}", grid[y as usize][x as usize]);
                    if seen.insert((x, y)) {
                        current_batch.insert((x, y));
                        niegh.iter().for_each(|(dir_x, dir_y)| {
                            let (side_x, side_y) = (x + dir_x, y + dir_y);
                            // We hit an edge
                            if side_x < 0
                                || side_x >= grid[0].len() as i32
                                || side_y < 0
                                || side_y >= grid.len() as i32
                            {
                                //println!("{side_x:?} {side_y:?} is edge");
                                // println!("Invalidate group: {current_batch:?}");
                                // panic!();
                                enclosed = false;
                                return;
                            }

                            let next_to = grid[side_y as usize][side_x as usize];

                            if !stack.contains(&(side_x, side_y))
                                && !seen.contains(&(side_x, side_y))
                                && !next_to.is_main_loop
                            {
                                // println!("Insert direct neighbors: {:?}", (side_x, side_y));
                                stack.push((side_x, side_y));
                            }

                            // We hit a pipe lets squeeze
                            if next_to.is_main_loop {
                                // Which side of the pipe are we ?
                                let mut dir = Direction::dir_to_pos(&-dir_x, &-dir_y);
                                // println!("Hit a pipe at {x} {y}: {next_to:?} we are {dir:?}");
                                dir = match next_to.c {
                                    'F' => Direction::UpLeft,
                                    'J' => Direction::DownRight,
                                    'L' => Direction::DownLeft,
                                    '7' => Direction::UpRigh,
                                    _ => dir,
                                };

                                let next = next_to.clone();
                                let mut optional = Some((next.after().unwrap(), next));

                                while let Some(((from_x, from_y), next_pipe)) = optional {
                                    // println!("Follow pipe {:?} {:?}", next_pipe, dir);
                                    seen_pipe_side.insert((next_pipe, dir));

                                    let poses_to_check =
                                        dir.get_neighbors(&next_pipe.pos.0, &next_pipe.pos.1);

                                    // println!("pos to check {poses_to_check:?}");
                                    match poses_to_check {
                                        (Some(p1), Some(p2)) => {
                                            if p1.0 >= 0
                                                && p1.0 < grid[0].len() as i32
                                                && p1.1 >= 0
                                                && p1.1 < grid.len() as i32
                                                && !grid[p1.1 as usize][p1.0 as usize].is_main_loop
                                                && !seen.contains(&p1)
                                                && !stack.contains(&p1)
                                            {
                                                // println!("Insert by queez {:?}", p1);
                                                stack.push(p1);
                                            }
                                            if p2.0 >= 0
                                                && p2.0 < grid[0].len() as i32
                                                && p2.1 >= 0
                                                && p2.1 < grid.len() as i32
                                                && !grid[p2.1 as usize][p2.0 as usize].is_main_loop
                                                && !seen.contains(&p2)
                                                && !stack.contains(&p2)
                                            {
                                                // println!("Insert by queez {:?}", p2);
                                                stack.push(p2);
                                            }
                                        }
                                        (Some(p1), None) => {
                                            if p1.0 >= 0
                                                && p1.0 < grid[0].len() as i32
                                                && p1.1 >= 0
                                                && p1.1 < grid.len() as i32
                                                && !grid[p1.1 as usize][p1.0 as usize].is_main_loop
                                                && !seen.contains(&p1)
                                                && !stack.contains(&p1)
                                            {
                                                // println!("Insert by queez {:?}", p1);
                                                stack.push(p1);
                                            }
                                        }
                                        _ => panic!(),
                                    };

                                    // Follow the pipe
                                    let (next_pos, by_dir) =
                                        next_pipe.pass_by(&from_x, &from_y).unwrap();
                                    let new_pipe = grid[next_pos.1 as usize][next_pos.0 as usize];

                                    dir = match (next_pipe.c, new_pipe.c) {
                                        ('-', '-') => dir,
                                        ('-', 'J') => {
                                            if dir == Direction::Up {
                                                Direction::UpLeft
                                            } else {
                                                Direction::DownRight
                                            }
                                        }
                                        ('-', '7') => {
                                            if dir == Direction::Up {
                                                Direction::UpRigh
                                            } else {
                                                Direction::DownLeft
                                            }
                                        }
                                        ('-', 'L') => {
                                            if dir == Direction::Up {
                                                Direction::UpRigh
                                            } else {
                                                Direction::DownLeft
                                            }
                                        }
                                        ('-', 'F') => {
                                            if dir == Direction::Up {
                                                Direction::UpLeft
                                            } else {
                                                Direction::DownRight
                                            }
                                        }

                                        ('|', '|') => dir,
                                        ('|', 'J') => {
                                            if dir == Direction::Left {
                                                Direction::UpLeft
                                            } else {
                                                Direction::DownRight
                                            }
                                        }
                                        ('|', '7') => {
                                            if dir == Direction::Left {
                                                Direction::DownLeft
                                            } else {
                                                Direction::UpRigh
                                            }
                                        }
                                        ('|', 'F') => {
                                            if dir == Direction::Left {
                                                Direction::UpLeft
                                            } else {
                                                Direction::DownRight
                                            }
                                        }
                                        ('|', 'L') => {
                                            if dir == Direction::Left {
                                                Direction::DownLeft
                                            } else {
                                                Direction::UpRigh
                                            }
                                        }
                                        // L
                                        ('L', '-') => {
                                            if dir == Direction::UpRigh {
                                                Direction::Up
                                            } else {
                                                Direction::Down
                                            }
                                        }
                                        ('L', 'F') => {
                                            if dir == Direction::UpRigh {
                                                Direction::DownRight
                                            } else {
                                                Direction::UpLeft
                                            }
                                        }
                                        ('L', '|') => {
                                            if dir == Direction::DownLeft {
                                                Direction::Left
                                            } else {
                                                Direction::Right
                                            }
                                        }
                                        ('L', 'J') => {
                                            if dir == Direction::UpRigh {
                                                Direction::UpLeft
                                            } else {
                                                Direction::DownLeft
                                            }
                                        }
                                        ('L', '7') => dir,

                                        // J
                                        ('J', 'L') => {
                                            if dir == Direction::UpLeft {
                                                Direction::UpRigh
                                            } else {
                                                Direction::DownLeft
                                            }
                                        }
                                        ('J', '-') => {
                                            if dir == Direction::UpLeft {
                                                Direction::Up
                                            } else {
                                                Direction::Down
                                            }
                                        }
                                        ('J', '|') => {
                                            if dir == Direction::UpLeft {
                                                Direction::Left
                                            } else {
                                                Direction::Right
                                            }
                                        }
                                        ('J', 'F') => dir,
                                        ('J', '7') => {
                                            if dir == Direction::UpLeft {
                                                Direction::DownLeft
                                            } else {
                                                Direction::UpRigh
                                            }
                                        }

                                        ('7', 'L') => dir,
                                        ('7', '-') => {
                                            if dir == Direction::UpRigh {
                                                Direction::Up
                                            } else {
                                                Direction::Down
                                            }
                                        }
                                        ('7', '|') => {
                                            if dir == Direction::UpRigh {
                                                Direction::Right
                                            } else {
                                                Direction::Left
                                            }
                                        }
                                        ('7', 'F') => {
                                            if dir == Direction::UpRigh {
                                                Direction::UpLeft
                                            } else {
                                                Direction::DownRight
                                            }
                                        }
                                        ('7', 'J') => {
                                            if dir == Direction::UpRigh {
                                                Direction::DownRight
                                            } else {
                                                Direction::UpLeft
                                            }
                                        }
                                        ('F', '-') => {
                                            if dir == Direction::UpLeft {
                                                Direction::Up
                                            } else {
                                                Direction::Down
                                            }
                                        }
                                        ('F', '|') => {
                                            if dir == Direction::UpLeft {
                                                Direction::Left
                                            } else if dir == Direction::DownRight {
                                                Direction::Right
                                            } else {
                                                panic!();
                                            }
                                        }
                                        ('F', 'J') => dir,
                                        ('F', '7') => {
                                            if dir == Direction::UpLeft {
                                                Direction::UpRigh
                                            } else {
                                                Direction::DownLeft
                                            }
                                        }
                                        ('F', 'L') => {
                                            if dir == Direction::UpLeft {
                                                Direction::DownLeft
                                            } else {
                                                Direction::UpRigh
                                            }
                                        }
                                        (a, b) => todo!("from {a} to {b}"),
                                    };

                                    if next_pos != next_to.pos
                                        && !seen_pipe_side.contains(&(new_pipe, dir))
                                    {
                                        optional = Some((next_pipe.pos, new_pipe));
                                    } else {
                                        optional = None;
                                    }
                                }
                            }
                        });
                    }
                }

                // println!("{current_batch:?} {enclosed}");
                if enclosed {
                    total_enclosed += current_batch.len();
                }
            }
        }
    }

    println!("p2: {total_enclosed:?}");

    Ok(())
}
