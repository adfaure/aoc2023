use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::BufRead;
use std::{fs::File, io::BufReader};
use rayon::prelude::*;

fn print_with_pose(grid: &Vec<Vec<char>>, reachable: HashSet<(i32, i32)>) {
    println!("{:?}", reachable);
    let min_x: i32 = reachable
        .iter()
        .map(|(x, _)| *x + 1)
        .min()
        .unwrap_or(0)
        .min(0);
    let max_x: i32 = reachable
        .iter()
        .map(|(x, _)| *x + 1)
        .max()
        .unwrap_or(grid[0].len() as i32)
        .max(grid[0].len() as i32);
    let min_y: i32 = reachable
        .iter()
        .map(|(_, y)| *y + 1)
        .min()
        .unwrap_or(0)
        .min(0);
    let max_y: i32 = reachable
        .iter()
        .map(|(_, y)| *y + 1)
        .max()
        .unwrap_or(grid.len() as i32)
        .max(grid.len() as i32);

    println!("---------{:?}{:?}-------", (min_x, max_x), (min_y, max_y));

    for y in min_y..max_y {
        for x in min_x..max_x {
            let pos = (x as usize % grid[0].len(), y as usize % grid.len());
            let c = grid[pos.1][pos.0];

            let (mut color_b, mut color_end) = ("", "");

            if x >= 0 && x < grid[0].len() as i32 && y >= 0 && y < grid.len() as i32 {
                color_b = "\x1b[93m";
                color_end = "\x1b[0m";
            }

            if reachable.contains(&(x as i32, y as i32)) && grid[pos.1][pos.0] != 'S' {
                print!("{}O{}", color_b, color_end)
            } else {
                print!("{}{c}{}", color_b, color_end)
            }
        }
        println!();
    }

    println!("-------------------------");
}

fn solve_p2(grid: &Vec<Vec<char>>, start: &Vec<(i32, i32)>, max_iter: u64) -> Vec<(u64, u64)> {
    let mut seen = HashSet::new();
    let mut fifo = VecDeque::new();
    let mut current_dist = 0;

    fifo.extend(start.iter().map(|pos| (0, *pos)));

    while let Some((dist, pos)) = fifo.pop_front() {
        if !seen.insert((dist, pos)) || dist > max_iter {
            continue;
        }

        if current_dist < dist {
            eprintln!("Finished {dist}");
            current_dist = dist;
        }

        [(0, 1), (0, -1), (1, 0), (-1, 0)]
            .iter()
            .map(|dir| ((pos.0 + dir.0), (pos.1 + dir.1)))
            .map(|pos| {
                let mut recalage = pos;

                if pos.0 >= grid[0].len() as i32 || pos.0 < 0 {
                    recalage.0 = recalage.0.rem_euclid(grid[0].len() as i32)
                }

                if pos.1 >= grid.len() as i32 || pos.1 < 0 {
                    recalage.1 = recalage.1.rem_euclid(grid.len() as i32)
                }

                // println!("{pos:?} => {recalage:?}: {:?} {}", pos.0 % grid[0].len() as i32, grid[0].len());
                (pos, recalage)
            })
            .filter(|(_pos, new_pos)| {
                    grid[new_pos.1 as usize][new_pos.0 as usize] == '.'
                        || grid[new_pos.1 as usize][new_pos.0 as usize] == 'S'
            })
            .for_each(|new_pos| {
                fifo.push_back((dist + 1, new_pos.0));
            });
    }

    seen
        .clone()
        .into_iter()
        .sorted_by(|a, b| a.0.cmp(&b.0))
        // .filter(|(dist, v)| *dist == max_iter)
        // .map(|(_, v)| v)
        .group_by(|elt| elt.0)
        .into_iter()
        .map(|(ge0, group)| (ge0 as u64, group.count() as u64))
        .collect::<Vec<(u64, u64)>>()
}

fn solve_p1(grid: &Vec<Vec<char>>, start: &Vec<(i32, i32)>, max_iter: u64) -> u64 {
    let mut seen = HashSet::new();
    let mut fifo = VecDeque::new();

    fifo.extend(start.iter().map(|pos| (0, *pos)));

    while let Some((dist, pos)) = fifo.pop_front() {
        if !seen.insert((dist, pos)) || dist > max_iter {
            continue;
        }

        [(0, 1), (0, -1), (1, 0), (-1, 0)]
            .iter()
            .map(|dir| ((pos.0 + dir.0), (pos.1 + dir.1)))
            .filter(|new_pos| {
                if new_pos.0 >= 0
                    && new_pos.1 >= 0
                    && new_pos.0 < grid[0].len() as i32
                    && new_pos.1 < grid.len() as i32
                {
                    grid[new_pos.1 as usize][new_pos.0 as usize] == '.'
                        || grid[new_pos.1 as usize][new_pos.0 as usize] == 'S'
                } else {
                    false
                }
            })
            .for_each(|new_pos| {
                fifo.push_back((dist + 1, new_pos));
            });

    }

    let reachable = seen
        .clone()
        .into_iter()
        // .filter(|(dist, v)| *dist == max_iter)
        .map(|(_, v)| v)
        .collect::<HashSet<_>>();

    // for (y, line) in grid.iter().enumerate() {
    //     for (x, c) in line.iter().enumerate() {
    //         if reachable.contains(&(x as i32, y as i32)) && grid[y][x] != 'S' {
    //             print!("O")
    //         } else {
    //             print!("{c}")
    //         }
    //     }
    //     println!();
    // }

    reachable.len() as u64
}

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
        .filter_map(|(y, line)| {
            line.iter().enumerate().find_map(|(x, c)| {
                if c == &'S' {
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .collect_vec();

    // let start = vec![(15, 0)];

    // println!("p1: {:?} ", solve_p1(&grid, &start, 64));
    println!("x;y");

    // let mut xs = vec![];
    // let mut ys = vec![];

    // (1..135).step_by(1).for_each(|i| {
    //     let re = solve_p2(&grid, &start, i) as f64;
    //     // xs.push(i as f64);
    //     // ys.push(re);
    //     println!("{i};{re}");
    // });

    // println!("500={:?}", solve_p2(&grid, &start, 50));

    for (dist, number) in solve_p2(&grid, &start, 982) {
        println!("{:?};{:?}", dist, number);
    }
    // println!("p2: {}", lagrange_interpolation(&xs, &ys, 1000f64));

    Ok(())
}
