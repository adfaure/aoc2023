use itertools::Itertools;
use std::collections::HashMap;
use std::fmt;
use std::io::BufRead;
use std::{fs::File, io::BufReader};

#[derive(Clone, Hash, Eq, PartialEq)]
struct ParabolicReflector {
    elements: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Tile {
    Round,
    Block,
    Empty,
}

impl ParabolicReflector {
    fn rows(
        &self,
    ) -> impl Iterator<
        Item = impl Iterator<Item = Tile> + '_ + DoubleEndedIterator + ExactSizeIterator,
    >
           + '_
           + DoubleEndedIterator
           + ExactSizeIterator {
        (0..self.height).map(move |y| (0..self.width).map(move |x| self.elements[y][x]))
    }
    fn columns(
        &self,
    ) -> impl Iterator<
        Item = impl Iterator<Item = Tile> + '_ + DoubleEndedIterator + ExactSizeIterator,
    >
           + '_
           + DoubleEndedIterator
           + ExactSizeIterator {
        (0..self.width).map(move |x| (0..self.height).map(move |y| self.elements[y][x]))
    }

    fn score_p1(&self) -> u32 {
        self.clone()
            .rows()
            .enumerate()
            .map(|row| {
                row.1
                    .enumerate()
                    .filter(|(_, t)| t == &Tile::Round)
                    .map(move |(x, t)| (x, row.0, t))
            })
            .flatten()
            // .inspect(|t| println!("{t:?}"))
            .map(|(_, y, _)| self.height - y)
            .sum::<usize>() as u32
    }

    fn tilt_east(&mut self) {
        self.clone()
            .columns()
            .enumerate()
            .rev()
            .map(|col| {
                col.1
                    .enumerate()
                    .filter(|(_, t)| t == &Tile::Round)
                    .map(move |(y, t)| (col.0, y, t))
            })
            .flatten()
            .for_each(|rounded| {
                // println!("rock: {rounded:?}");
                let rolling_poses = self
                    .rows()
                    .skip(rounded.1)
                    .take(1)
                    .flatten()
                    .skip(rounded.0 + 1)
                    // .inspect(|t| print!("{:?}", t))
                    .take_while(|c| c == &Tile::Empty)
                    .count();

                // println!("\nrol to: {:?}", rolling_poses);
                self.elements[rounded.1][rounded.0] = Tile::Empty;
                self.elements[rounded.1][rounded.0 + rolling_poses] = Tile::Round;
            });
    }

    fn tilt_south(&mut self) {
        self.clone()
            .rows()
            .enumerate()
            .rev()
            .map(|row| {
                row.1
                    .enumerate()
                    .filter(|(_, t)| t == &Tile::Round)
                    .map(move |(x, t)| (x, row.0, t))
            })
            .flatten()
            .for_each(|rounded| {
                let rolling_poses = self
                    .columns()
                    .skip(rounded.0)
                    .take(1)
                    .flatten()
                    .skip(rounded.1 + 1)
                    // .inspect(|t| print!("{:?}", t))
                    .take_while(|c| c == &Tile::Empty)
                    .count();

                // println!("\nrol to: {:?}", rolling_poses);
                self.elements[rounded.1][rounded.0] = Tile::Empty;
                self.elements[rounded.1 + rolling_poses][rounded.0] = Tile::Round;
            });
    }

    fn tilt_west(&mut self) {
        self.clone()
            .columns()
            .enumerate()
            .map(|col| {
                col.1
                    .enumerate()
                    .filter(|(_, t)| t == &Tile::Round)
                    .map(move |(y, t)| (col.0, y, t))
            })
            .flatten()
            .for_each(|rounded| {
                // println!("rock: {rounded:?}");
                let rolling_poses = self
                    .rows()
                    .skip(rounded.1)
                    .take(1)
                    .rev()
                    .flatten()
                    .rev()
                    .skip(self.width - rounded.0)
                    // .inspect(|t| print!("{:?}", t))
                    .take_while(|c| c == &Tile::Empty)
                    .count();

                // println!("\nrol to: {:?}", rolling_poses);
                self.elements[rounded.1][rounded.0] = Tile::Empty;
                self.elements[rounded.1][rounded.0 - rolling_poses] = Tile::Round;
            });
    }

    fn tilt_north(&mut self) {
        self.clone()
            .rows()
            .enumerate()
            .map(|row| {
                row.1
                    .enumerate()
                    .filter(|(_, t)| t == &Tile::Round)
                    .map(move |(x, t)| (x, row.0, t))
            })
            .flatten()
            .for_each(|rounded| {
                let rolling_poses = self
                    .columns()
                    .skip(rounded.0)
                    .take(1)
                    .rev()
                    .flatten()
                    .rev()
                    .skip(self.height - rounded.1)
                    .take_while(|c| c == &Tile::Empty)
                    .count();

                self.elements[rounded.1][rounded.0] = Tile::Empty;
                self.elements[rounded.1 - rolling_poses][rounded.0] = Tile::Round;
            });
    }
}

impl From<char> for Tile {
    fn from(item: char) -> Self {
        match item {
            '.' => Tile::Empty,
            '#' => Tile::Block,
            'O' => Tile::Round,
            _ => panic!("Invalid char: {:?}", item),
        }
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Tile::Round => 'O',
            Tile::Block => '#',
            Tile::Empty => '.',
        };
        write!(f, "{c}")
    }
}

fn main() -> std::io::Result<()> {
    let pr = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.chars().map(|c| Tile::from(c)).collect_vec())
        .collect_vec();

    let mut parabolic_reflector = ParabolicReflector {
        width: pr[0].len(),
        height: pr.len(),
        elements: pr,
    };

    // parabolic_reflector.rows().for_each(|col| {
    //     col.for_each(|e| {
    //         print!("{e:?}");
    //     });
    //     println!();
    // });

    let mut p1 = parabolic_reflector.clone();
    p1.tilt_north();
    println!("p1 {}", p1.score_p1());

    println!("----");
    // parabolic_reflector.tilt_west();
    // parabolic_reflector.rows().for_each(|col| {
    //     col.for_each(|e| {
    //         print!("{e:?}");
    //     });
    //     println!();
    // });

    let iter = 1_000_000_000;

    let mut memo = HashMap::new();
    let mut find_cycle_grid = parabolic_reflector.clone();

    let cycle = (0..).find_map(|i| {

        match memo.insert(find_cycle_grid.clone(), i) {
            Some(cycle_start) => {
                println!("{:?}", i);
                return Some((cycle_start, i - cycle_start))
            },
            None => {}
        };

        find_cycle_grid.tilt_north();
        find_cycle_grid.tilt_west();
        find_cycle_grid.tilt_south();
        find_cycle_grid.tilt_east();

        None
    }).unwrap();


    let todo = cycle.0 + ((iter - cycle.0) % cycle.1);
    println!("cycle at {cycle:?} - nb cycle to do {}", todo);

    (0..todo).for_each(|_| {
        // println!("{i} : {}", parabolic_reflector.score_p1());

        parabolic_reflector.tilt_north();
        parabolic_reflector.tilt_west();
        parabolic_reflector.tilt_south();
        parabolic_reflector.tilt_east();

    });

    println!("p2: {}", parabolic_reflector.score_p1());

    Ok(())
}

// println!("is solution: {}", is_solution);
