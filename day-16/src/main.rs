use std::collections::HashSet;
use std::io::BufRead;
use std::{fs::File, io::BufReader};

#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    fn to_vec(&self) -> (i32, i32) {
        match self {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
        }
    }
    fn rotate(&self, alpha: f32) -> Direction {
        let (x, y) = self.to_vec();

        let x = x as f32;
        let y = y as f32;

        let res = (
            (x * alpha.cos() - y * alpha.sin()).round() as i32,
            (x * alpha.sin() + y * alpha.cos()).round() as i32,
        );

        Direction::from_vec(res)
    }
    fn from_vec(pos: (i32, i32)) -> Direction {
        match pos {
            (-1, 0) => Direction::Left,
            (1, 0) => Direction::Right,
            (0, -1) => Direction::Up,
            (0, 1) => Direction::Down,
            _ => panic!(),
        }
    }
}

fn main() -> std::io::Result<()> {
    /* part 1 */
    let grid: Vec<Vec<char>> = BufReader::new(File::open("input")?)
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect::<Vec<Vec<_>>>();


    let tries = (0..grid.len())
        .flat_map(|y| [((-1, y as i32), Direction::Right), ((grid[0].len() as i32, y as i32) , Direction::Left)].into_iter())
        .chain(
            (0..grid[0].len())
                .flat_map(|x| [((x as i32, -1), Direction::Down), ((x as i32, grid.len() as i32), Direction::Up)].into_iter()),
        );

    let mut p1 = 0;

    let res: u32 = tries
        .map(|(start_pos, start_dir)| {
            let beam_start = (start_pos, start_dir, 0);

            let mut infitine_detector: HashSet<(i32, i32, Direction)> = HashSet::new();
        
            let mut beam_stack = [beam_start].to_vec();
            let mut seen: HashSet<(i32, i32)> = HashSet::new();

            while let Some(current_beam) = beam_stack.pop() {
                seen.insert(current_beam.0);

                if !infitine_detector.insert((current_beam.0 .0, current_beam.0 .1, current_beam.1))
                {
                    // Fading beam already seen this configuration
                    continue;
                }

                let beam_dir = current_beam.1.to_vec();
                let next_tile = (
                    current_beam.0 .0 + beam_dir.0,
                    current_beam.0 .1 + beam_dir.1,
                );

                match (
                    current_beam.1,
                    grid.get(next_tile.1 as usize)
                        .and_then(|line| line.get(next_tile.0 as usize)),
                ) {
                    (_, Some('.')) => beam_stack.push((next_tile, current_beam.1, current_beam.2)),
                    (Direction::Left, Some('|')) | (Direction::Right, Some('|')) => {
                        beam_stack.push((next_tile, Direction::Up, current_beam.2));
                        beam_stack.push((next_tile, Direction::Down, current_beam.2 + 1));
                    }
                    (Direction::Up, Some('-')) | (Direction::Down, Some('-')) => {
                        beam_stack.push((next_tile, Direction::Left, current_beam.2));
                        beam_stack.push((next_tile, Direction::Right, current_beam.2 + 1));
                    }
                    (dir, Some('-')) |  (dir, Some('|')) => {
                        beam_stack.push((next_tile, dir, current_beam.2));
                    }
                    
                    (dir, Some('\\')) if dir == Direction::Up || dir == Direction::Down => {
                        beam_stack.push((next_tile, Direction::rotate(&dir, -90.), current_beam.2));
                    }
                    (dir, Some('\\')) if dir == Direction::Right || dir == Direction::Left => {
                        beam_stack.push((next_tile, Direction::rotate(&dir, 90.), current_beam.2));
                    }
                    (dir, Some('/')) if dir == Direction::Up || dir == Direction::Down => {
                        beam_stack.push((next_tile, Direction::rotate(&dir, 90.), current_beam.2));
                    }
                    (dir, Some('/')) if dir == Direction::Right || dir == Direction::Left => {
                        beam_stack.push((next_tile, Direction::rotate(&dir, -90.), current_beam.2));
                    }
                    (_, None) => {} // Beam fading
                    (a, b) => panic!("{a:?} {b:?}"),
                }
                // println!("stack size {}", beam_stack.len());
            }

            if beam_start.0 == (-1, 0) {
                p1 = seen.len();
            }
 
            seen.len() as u32
        })
        .max()
        .unwrap();

    // Remove 1 bc of the starting point at (-1, 0)
    println!("p1: {}", p1 - 1);
    println!("p2: {}", res - 1);

    Ok(())
}
