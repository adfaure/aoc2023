use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;
use std::io::BufRead;
use std::str::FromStr;
use std::time::Instant;
use std::{fs::File, io::BufReader};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Shape {
    c1: (i32, i32, i32),
    c2: (i32, i32, i32),
}

impl Shape {
    fn _cubes(&self) -> impl Iterator<Item = (i32, i32, i32)> + Clone + '_ {
        let (from, to);
        if self.c1.1 != self.c2.1 {
            (from, to) = (self.c1.1.min(self.c2.1), self.c1.1.max(self.c2.1));
        } else if self.c1.0 != self.c2.0 {
            (from, to) = (self.c1.0.min(self.c2.0), self.c1.0.max(self.c2.0));
        } else if self.c1.2 != self.c2.2 {
            (from, to) = (self.c1.2.min(self.c2.2), self.c1.2.max(self.c2.2));
        } else {
            (from, to) = (0, 0);
        }

        (from..=to).map(move |x_or_y| {
            if self.c1.0 != self.c2.0 {
                (x_or_y, self.c1.1, self.c1.2)
            } else if self.c1.1 != self.c2.1 {
                (self.c1.0, x_or_y, self.c1.2)
            } else if self.c1.2 != self.c2.2 {
                (self.c1.0, self.c1.1, x_or_y)
            } else {
                (self.c1.0, self.c1.1, self.c1.2)
            }
        })
    }

    fn move_dir(&mut self, (x, y, z): (i32, i32, i32)) {
        self.c1 = (self.c1.0 + x, self.c1.1 + y, self.c1.2 + z);
        self.c2 = (self.c2.0 + x, self.c2.1 + y, self.c2.2 + z);
    }

    fn collides(&self, other: &Shape) -> bool {
        if self.c1.1 != self.c2.1 {
            let (s_min, s_max) = (self.c2.1.min(self.c1.1), self.c2.1.max(self.c1.1));
            if other.c1.1 != other.c2.1 {
                let (o_min, o_max) = (other.c2.1.min(other.c1.1), other.c2.1.max(other.c1.1));
                return self.c1.0 == other.c1.0
                    && self.c1.2 == other.c1.2
                    && (s_min <= o_max && o_min <= s_max);
            } else if other.c1.0 != other.c2.0 {
                let (o_min, o_max) = (other.c2.0.min(other.c1.0), other.c2.0.max(other.c1.0));
                return (s_min..=s_max).contains(&other.c1.1)
                    && (o_min..=o_max).contains(&self.c1.0)
                    && self.c1.2 == other.c1.2;
            } else if other.c1.2 != other.c2.2 {
                let (o_min, o_max) = (other.c2.2.min(other.c1.2), other.c2.2.max(other.c1.2));
                return (s_min..=s_max).contains(&other.c1.1)
                    && (o_min..=o_max).contains(&self.c1.2)
                    && self.c1.0 == other.c1.0;
            } else {
                return (s_min..=s_max).contains(&other.c1.1)
                    && self.c1.2 == other.c1.2
                    && self.c1.0 == other.c1.0;
            }
        } else if self.c1.0 != self.c2.0 {
            let (s_min, s_max) = (self.c2.0.min(self.c1.0), self.c2.0.max(self.c1.0));
            if other.c1.1 != other.c2.1 {
                let (o_min, o_max) = (other.c2.1.min(other.c1.1), other.c2.1.max(other.c1.1));
                return (o_min..=o_max).contains(&self.c1.1)
                    && (s_min..=s_max).contains(&other.c1.0)
                    && self.c1.2 == other.c1.2;
            } else if other.c1.0 != other.c2.0 {
                let (o_min, o_max) = (other.c2.0.min(other.c1.0), other.c2.0.max(other.c1.0));
                return self.c1.1 == other.c1.1
                    && self.c1.2 == other.c1.2
                    && (s_min <= o_max && o_min <= s_max);
            } else if other.c1.2 != other.c2.2 {
                let (o_min, o_max) = (other.c2.2.min(other.c1.2), other.c2.2.max(other.c1.2));
                return (s_min..=s_max).contains(&other.c1.0)
                    && (o_min..=o_max).contains(&self.c1.2)
                    && self.c1.1 == other.c1.1;
            } else {
                return (s_min..=s_max).contains(&other.c1.0)
                    && self.c1.2 == other.c1.2
                    && self.c1.1 == other.c1.1;
            }
        } else if self.c1.2 != self.c2.2 {
            let (s_min, s_max) = (self.c2.2.min(self.c1.2), self.c2.2.max(self.c1.2));
            if other.c1.1 != other.c2.1 {
                let (o_min, o_max) = (other.c2.1.min(other.c1.1), other.c2.1.max(other.c1.1));
                return (o_min..=o_max).contains(&self.c1.1)
                    && (s_min..=s_max).contains(&other.c1.2)
                    && self.c1.0 == other.c1.0;
            } else if other.c1.0 != other.c2.0 {
                let (o_min, o_max) = (other.c2.0.min(other.c1.0), other.c2.0.max(other.c1.0));
                return (o_min..=o_max).contains(&self.c1.0)
                    && (s_min..=s_max).contains(&other.c1.2)
                    && self.c1.1 == other.c1.1;
            } else if other.c1.2 != other.c2.2 {
                let (o_min, o_max) = (other.c2.2.min(other.c1.2), other.c2.2.max(other.c1.2));
                return self.c1.1 == other.c1.1
                    && self.c1.0 == other.c1.0
                    && (s_min <= o_max && o_min <= s_max);
            } else {
                return (s_min..=s_max).contains(&other.c1.2)
                    && self.c1.0 == other.c1.0
                    && self.c1.1 == other.c1.1;
            }
        } else {
            if other.c1.1 != other.c2.1 {
                let (o_min, o_max) = (other.c2.1.min(other.c1.1), other.c2.1.max(other.c1.1));
                return (o_min..=o_max).contains(&self.c1.1)
                    && self.c1.2 == other.c1.2
                    && self.c1.0 == other.c1.0;
            } else if other.c1.0 != other.c2.0 {
                let (o_min, o_max) = (other.c2.0.min(other.c1.0), other.c2.0.max(other.c1.0));
                return (o_min..=o_max).contains(&self.c1.0)
                    && self.c1.2 == other.c1.2
                    && self.c1.1 == other.c1.1;
            } else if other.c1.2 != other.c2.2 {
                let (o_min, o_max) = (other.c2.2.min(other.c1.2), other.c2.2.max(other.c1.2));
                return self.c1.1 == other.c1.1
                    && self.c1.0 == other.c1.0
                    && (o_min..=o_max).contains(&self.c1.2);
            } else {
                return self == other;
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseModuleError;

impl FromStr for Shape {
    type Err = ParseModuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r_parse = Regex::new(r"(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)$").unwrap();

        if r_parse.is_match(s) {
            let (x1, y1, z1, x2, y2, z2) = r_parse
                .captures_iter(&s)
                .next()
                .unwrap()
                .extract::<6>()
                .1
                .iter()
                .map(|c| c.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap();

            return Ok(Shape {
                c1: (x1, y1, z1),
                c2: (x2, y2, z2),
            });
        }
        Err(ParseModuleError)
    }
}

fn safe_to_disintegrate(shapes: &Vec<Shape>, idx: usize) -> bool {
    let mut sim_removal = shapes.clone();
    sim_removal.remove(idx);

    for shape in sim_removal.iter() {
        let mut try_move: Shape = shape.clone();
        try_move.move_dir((0, 0, -1));

        if try_move.c1.2.min(try_move.c2.2) >= 1
            && sim_removal
                .iter()
                .filter(|other| other != &shape)
                .all(|other| !other.collides(&try_move))
        {
            return false;
        }
    }

    return true;
}

fn disintegratation_rate(shapes: &Vec<Shape>, idx: usize) -> u32 {
    let mut sim_removal = shapes.clone();
    sim_removal.remove(idx);

    let mut score = 0;
    let mut res = sim_removal.clone();

    for idx in 0..sim_removal.len() {
        // println!("{:?}", res);
        let mut can_fall = true;
        let mut falled = false;
        let mut try_move: Shape = res.remove(idx);

        while try_move.c1.2.min(try_move.c2.2) > 1 && can_fall {
            try_move.move_dir((0, 0, -1));
            for other_idx in 0..(sim_removal.len() - 1) {
                if res[other_idx].collides(&try_move) {
                    try_move.move_dir((0, 0, 1));
                    can_fall = false;
                    break;
                }
            }

            if can_fall {
                falled = true;
            }
        }

        if falled {
            score += 1;
        }

        res.insert(idx, try_move);
    }

    score
}

fn solve_p1(mut shapes: Vec<Shape>) {
    let now = Instant::now();
    // sort by z axis, make the lowest fall first
    shapes.sort_by(|a, b| a.c1.2.min(a.c2.2).cmp(&b.c1.2.min(b.c2.2)));
    let mut res = shapes.clone();

    for idx in 0..shapes.len() {
        // println!("{:?}", res);
        let mut can_fall = true;
        let mut try_move: Shape = res.remove(idx);

        while try_move.c1.2.min(try_move.c2.2) > 1 && can_fall {
            try_move.move_dir((0, 0, -1));

            for other_idx in 0..(shapes.len() - 1) {
                if res[other_idx].collides(&try_move) {
                    try_move.move_dir((0, 0, 1));
                    can_fall = false;
                    break;
                }
            }
        }

        res.insert(idx, try_move);
    }

    println!(
        "p2: {:?} in {:?}",
        (0..res.len())
            .par_bridge()
            .filter(|idx| safe_to_disintegrate(&res, *idx))
            .count(),
        now.elapsed()
    );
}

fn solve_p2(mut shapes: Vec<Shape>) {
    let now = Instant::now();

    // sort by z axis, make the lowest fall first
    shapes.sort_by(|a, b| a.c1.2.min(a.c2.2).cmp(&b.c1.2.min(b.c2.2)));
    let mut res = shapes.clone();

    for idx in 0..shapes.len() {
        // println!("{:?}", res);
        let mut can_fall = true;
        let mut try_move: Shape = res.remove(idx);

        while try_move.c1.2.min(try_move.c2.2) > 1 && can_fall {
            try_move.move_dir((0, 0, -1));

            for other_idx in 0..(shapes.len() - 1) {
                if res[other_idx].collides(&try_move) {
                    try_move.move_dir((0, 0, 1));
                    can_fall = false;
                    break;
                }
            }
        }

        res.insert(idx, try_move);
    }

    println!(
        "p2: {:?} in {:?}",
        (0..res.len())
            .par_bridge()
            .map(|idx| disintegratation_rate(&res, idx))
            .sum::<u32>(),
        now.elapsed()
    );
}

fn main() -> std::io::Result<()> {
    /* part 1 */

    let shapes = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| line != "")
        .filter_map(|line| Shape::from_str(&line).ok())
        .collect_vec();

    solve_p1(shapes.clone());

    solve_p2(shapes);
    Ok(())
}
