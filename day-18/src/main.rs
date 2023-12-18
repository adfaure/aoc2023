use itertools::Itertools;
use rand::prelude::*;
use regex::Regex;
use std::collections::HashSet;
use std::io::BufRead;
use std::{fs::File, io::BufReader};

fn main() -> std::io::Result<()> {
    /* part 1 */
    let r = Regex::new(r"(.) (\d+) (\(#......\))$").unwrap();

    let trenchs = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| r.is_match(&line))
        .map(|line| {
            let (d, n, c) = r
                .captures_iter(&line)
                .map(|capture| {
                    capture
                        .iter()
                        .skip(1)
                        .filter_map(|m| m)
                        .map(|m| m.as_str())
                        .collect_vec()
                })
                .flatten()
                .map(String::from)
                .collect_tuple()
                .unwrap();

            (d, n.parse::<i32>().unwrap())
        })
        .collect::<Vec<_>>();

    let (mut min_x, mut max_x) = (0, 0);
    let mut cur_size_x: i32 = 0;
    let mut cur_size_y: i32 = 0;
    let (mut min_y, mut max_y) = (0, 0);

    for (d, n) in trenchs.iter() {
        match d.as_str() {
            "R" => {
                cur_size_x += n;
                max_x = max_x.max(cur_size_x);
            }
            "L" => {
                cur_size_x -= n;
                min_x = min_x.min(cur_size_x);
            }
            "D" => {
                cur_size_y += n;
                max_y = max_y.max(cur_size_y);
            }
            "U" => {
                cur_size_y -= n;
                min_y = min_y.min(cur_size_y);
            }
            _ => panic!(),
        }
    }

    let size_x = max_x - min_x;
    let size_y = max_y - min_y;
    println!(
        "map size: {:?} {:?} {:?}",
        (min_x, max_x),
        (min_y, max_y),
        (size_x, size_y)
    );

    let line = vec!['.'; 1 + size_x as usize];
    let mut grid = vec![line.clone(); 1 + size_y as usize];
    let mut current_pos = (0, min_y.abs() as usize);
    println!("starts at: {:?}", current_pos);

    for (d, n) in trenchs.iter() {
        for _ in 0..(*n as usize) {
            match d.as_str() {
                "R" => {
                    let fixed = current_pos.1;
                    current_pos.0 += 1;
                    grid[fixed][current_pos.0] = '#';
                }
                "L" => {
                    let fixed = current_pos.1;
                    current_pos.0 -= 1;
                    grid[fixed][current_pos.0] = '#';
                }
                "D" => {
                    let fixed = current_pos.0;
                    current_pos.1 += 1;
                    grid[current_pos.1][fixed] = '#';
                }
                "U" => {
                    let fixed = current_pos.0;
                    current_pos.1 -= 1;
                    grid[current_pos.1][fixed] = '#';
                }
                _ => panic!(),
            }
        }
    }

    // grid.iter().for_each(|line| {
    //     println!("{}", line.iter().join(""));
    // });

    let mut stack = Vec::new();
    let mut seen = HashSet::new();

    let mut rng = rand::thread_rng();

    let start = (0 as usize, 3 as usize);
    stack.push(start);

    while let Some(current) = stack.pop() {
        let mut exit_current = false;
        for (neigh_x, neigh_y) in [(0, 1), (0, -1), (1, 0), (-1, 0)].into_iter() {
            let neigh = (
                current.0.checked_add_signed(neigh_x),
                current.1.checked_add_signed(neigh_y),
            );

            match neigh {
                (Some(x), Some(y)) => {
                    let at_pos = grid
                        .get(neigh.1.unwrap())
                        .and_then(|line| line.get(neigh.0.unwrap()));
                    match at_pos {
                        Some('.') if seen.insert((x, y)) => {
                            stack.push((x, y));
                        }
                        None => {
                            exit_current = true;
                            break;
                        }
                        _ => {}
                    }
                }
                cords => {
                    // reaching edge, not inside
                    exit_current = true;
                    break;
                }
            }
        }

        if exit_current {
            seen.clear();
            stack.clear();

            let new_start = (
                rng.gen::<usize>() % size_x as usize,
                rng.gen::<usize>() % size_y as usize,
            );
            println!("cleared: {new_start:?}");
            stack.push(new_start)
        }
    }

    let trenches_size = trenchs.iter().fold(0, |acc, (_, n)| acc + n);

    println!("p1: {:?}", seen.len() + trenches_size as usize);

    Ok(())
}
