use itertools::Itertools;
use std::fmt;
use std::io::BufRead;
use std::{fs::File, io::BufReader};

#[derive(Clone)]
struct ParabolicReflector {
    elements: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
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
    fn columns_v2(&self) -> impl Iterator<Item = Vec<Tile>> + '_ {
        (0..self.width).map(move |x| {
            (0..self.height)
                .map(move |y| self.elements[y][x])
                .collect_vec()
        })
    }

    fn score_p1(&self) -> u32 {
        self
            .clone()
            .rows()
            .enumerate()
            .map(|row| {
                row.1
                    .enumerate()
                    .filter(|(_, t)| t == &Tile::Round)
                    .map(move |(x, t)| (x, row.0, t))
            })
            .flatten()
            .inspect(|t| println!("{t:?}"))
            .map(|(x, y, _)| self.height - y)
            .sum::<usize>() as u32

    }

    fn tilt_north(&mut self) {
        let rock_ordered = self
            .clone()
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
                // println!("{rounded:?}");
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

                println!("rol to: {:?}", rolling_poses);
                self.elements[rounded.1][rounded.0] = Tile::Empty;
                self.elements[rounded.1 - rolling_poses][rounded.0] = Tile::Round;
            });
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Tile {
    Round,
    Block,
    Empty,
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
        width: pr.len(),
        height: pr[0].len(),
        elements: pr,
    };

    parabolic_reflector.rows().for_each(|col| {
        col.for_each(|e| {
            print!("{e:?}");
        });
        println!();
    });

    parabolic_reflector.tilt_north();

    parabolic_reflector.rows().for_each(|col| {
        col.for_each(|e| {
            print!("{e:?}");
        });
        println!();
    });

    println!("p1 {}", parabolic_reflector.score_p1());
    Ok(())
}

// println!("is solution: {}", is_solution);
