use itertools::Itertools;
use regex::Regex;
use std::cell::Cell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::BufRead;
use std::rc::Rc;
use std::str::FromStr;
use std::{fs::File, io::BufReader};

fn main() -> std::io::Result<()> {
    /* part 1 */

    let grid = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| line != "")
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let start = grid
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter().enumerate().find_map(|(x, c)| {
                if c == &'S' {
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .unwrap();

    let mut seen = HashSet::new();
    let mut fifo = VecDeque::new();

    fifo.push_back((0, start));

    let max_iter = 64;

    while let Some((dist, pos)) = fifo.pop_front() {
        if !seen.insert((dist, pos)) || dist > max_iter {
            continue;
        }

        [(0, 1), (0, -1), (1, 0), (-1, 0)]
            .iter()
            .map(|dir| ((pos.0 + dir.0) as i32, (pos.1 + dir.1) as i32))
            .filter(|new_pos| {
                if new_pos.0 >= 0
                    && new_pos.1 >= 0
                    && new_pos.0 < grid[0].len() as i32
                    && new_pos.1 < grid.len() as i32
                {
                    return grid[new_pos.1 as usize][new_pos.0 as usize] == '.'
                        || grid[new_pos.1 as usize][new_pos.0 as usize] == 'S';
                } else {
                    return false;
                }
            })
            .for_each(|new_pos| {
                fifo.push_back((dist + 1, new_pos));
            });
    }

    let reachable = seen
        .clone()
        .into_iter()
        .filter(|(dist, v)| *dist == max_iter)
        .map(|(_, v)| v)
        .collect::<HashSet<_>>();

    // for (y, line) in grid.iter().enumerate() {
    //     for (x, c) in line.iter().enumerate() {
    //         if reachable.contains(&(x as i32, y as i32)) {
    //             print!("O")
    //         } else {
    //             print!("{c}")
    //         }
    //     }
    //     println!();
    // }

    // println!("{seen:?}");

    println!(
        "p1: {:?} ",
        reachable.len()
    );
    Ok(())
}
