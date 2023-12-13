use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::io::BufRead;
use std::iter;
use std::{fs::File, io::BufReader};

fn main() -> std::io::Result<()> {
    let problems: usize = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|line| line.ok())
        .batching(|lineiter| {
            let grid: Vec<String> = lineiter.take_while(|line| line.len() != 0).collect_vec();
            if grid.is_empty() {
                return None;
            }
            Some(grid)
        })
        .map(|grid_lines: Vec<String>| {
            grid_lines
                .into_iter()
                .map(|line| line.chars().collect_vec())
                .collect_vec()
        })
        // .inspect(|l| println!("{:?}", l))
        .map(|grid| {
            // create a transpose
            let mut transposed = iter::repeat([].to_vec()).take(grid[0].len()).collect_vec();
            for x in 0..grid[0].len() {
                for y in 0..grid.len() {
                    transposed[x].push(grid[y][x])
                }
            }

            // grid
            //     .iter()
            //     .inspect(|l| println!("grid: {}", l.iter().join("")))
            //     .for_each(|_| {});

            // transposed
            //     .iter()
            //     .inspect(|l| println!("t_grid: {}", l.iter().join("")))
            //     .for_each(|_| {});

            [transposed, grid]
                .iter()
                .enumerate()
                .find_map(|(idx, grid)| {
                    // start with lines
                    let res = (0..grid.len()).find_map(|i| {
                        let (up, down) = grid.split_at(i);
                        // println!("idx: {i} => \n  up: {up:?}\ndown: {down:?}");

                        if up
                            .iter()
                            .rev()
                            .zip_longest(down.iter())
                            .fold_while(false, |acc, zipps| match zipps {
                                Both(up, down) => {
                                    if up == down {
                                        Continue(true)
                                    } else {
                                        Done(false)
                                    }
                                }
                                Left(_) | Right(_) => Done(acc),
                            })
                            .into_inner()
                        {
                            return Some(i);
                        } else {
                            return None;
                        }
                    });

                    match res {
                        Some(row) => {
                            if idx == 0 {
                                return Some(row);
                            } else {
                                return Some(row * 100);
                            }
                        }
                        None => None,
                    }
                })
                .unwrap_or(0)
        })
        // .inspect(|a| println!("{:?}", a))
        .sum();

    println!("p1 {:?}", problems);

    let problems: usize = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|line| line.ok())
        .batching(|lineiter| {
            let grid: Vec<String> = lineiter.take_while(|line| line.len() != 0).collect_vec();
            if grid.is_empty() {
                return None;
            }
            Some(grid)
        })
        .map(|grid_lines: Vec<String>| {
            grid_lines
                .into_iter()
                .map(|line| line.chars().collect_vec())
                .collect_vec()
        })
        // .inspect(|l| println!("{:?}", l))
        .map(|grid| {
            // create a transpose
            let mut transposed = iter::repeat([].to_vec()).take(grid[0].len()).collect_vec();
            for x in 0..grid[0].len() {
                for y in 0..grid.len() {
                    transposed[x].push(grid[y][x])
                }
            }

            // grid
            //     .iter()
            //     .inspect(|l| println!("grid: {}", l.iter().join("")))
            //     .for_each(|_| {});

            // transposed
            //     .iter()
            //     .inspect(|l| println!("t_grid: {}", l.iter().join("")))
            //     .for_each(|_| {});

            [transposed, grid]
                .iter()
                .enumerate()
                .find_map(|(idx, grid)| {
                    // start with lines
                    let res = (0..grid.len()).find_map(|i| {
                        let (up, down) = grid.split_at(i);
                        // println!("idx: {i} => \n  up: {up:?}\ndown: {down:?}");

                        if up
                            .iter()
                            .rev()
                            .zip_longest(down.iter())
                            .fold_while((false,false), |(acc, found_pix), zipps| match zipps {
                                Both(up, down) => {
                                    if up == down {
                                        Continue((true, found_pix))
                                    } else {
                                        let dead_pix =
                                            up.iter().zip(down).fold(0, |acc, (p_up, p_down)| {
                                                if p_up == p_down {
                                                    acc
                                                } else {
                                                    acc + 1
                                                }
                                            });

                                        if dead_pix == 1 && !found_pix {
                                            Continue((true, true))
                                        } else {
                                            Done((false, found_pix))
                                        }
                                    }
                                }

                                Left(_) | Right(_) => Done((acc && found_pix, found_pix)),
                            })
                            .into_inner().0
                        {
                            return Some(i);
                        } else {
                            return None;
                        }
                    });

                    match res {
                        Some(row) => {
                            if idx == 0 {
                                return Some(row);
                            } else {
                                return Some(row * 100);
                            }
                        }
                        None => None,
                    }
                })
                .unwrap_or(0)
        })
        .sum();

    println!("p2 {:?}", problems);
    Ok(())
}

// println!("is solution: {}", is_solution);
