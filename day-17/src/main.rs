use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::BufRead;
use std::{fs::File, io::BufReader};

use itertools::Itertools;

fn reverse_dir(dir: &(i32, i32)) -> (i32, i32) {
    (dir.0 * -1, dir.1 * -1)
}

fn grid_print(grid: &Vec<Vec<u32>>, start: &(u32, u32), path: &Vec<(i32, i32)>) {
    let poses = path
        .iter()
        .fold([(*start, (0, 0))].to_vec(), |mut acc, dir| {
            let last = acc.last().unwrap().0;
            let next_pos = (
                last.0.checked_add_signed(dir.0).unwrap(),
                last.1.checked_add_signed(dir.1).unwrap(),
            );
            acc.push((next_pos, *dir));

            acc
        });

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            match poses.iter().find(|pos| pos.0 == (x as u32, y as u32)) {
                Some((_, dir)) => match dir {
                    (0, -1) => print!("^"),
                    (0, 1) => print!("v"),
                    (1, 0) => print!(">"),
                    (-1, 0) => print!("<"),
                    (0, 0) => print!("S"),
                    dir => panic!("{dir:?}"),
                },
                None => print!("{}", grid[y][x]),
            };
        }
        println!()
    }
}

fn neighbors(grid: &Vec<Vec<u32>>, pos: (u32, u32)) -> impl Iterator<Item = (u32, u32)> + '_ {
    [(0, 1), (0, -1), (1, 0), (-1, 0)]
        .into_iter()
        .filter_map(move |(x_dir, y_dir)| {
            match (
                pos.0.checked_add_signed(x_dir),
                pos.1.checked_add_signed(y_dir),
            ) {
                (Some(x), Some(y)) if (y as usize) < grid.len() && (x as usize) < grid[0].len() => {
                    Some((x, y))
                }
                _ => None,
            }
        })

    // ((pos.0.checked_sub(1)).unwrap_or(0)..=(pos.0 + 1))
    //     .map(move |x| ((pos.1.checked_sub(1)).unwrap_or(0)..=(pos.1 + 1)).map(move |y| (x, y)))
    //     .flatten()
    //     .filter(move |(x, y)| {
    //         x != y && x >= &0 && *x < grid[0].len() as u32 && y >= &0 && *y < grid.len() as u32
    //     })
}

#[derive(PartialEq, Eq, Debug)]
struct HeuristicCell<T: Eq> {
    weight: u32,
    heuristics: u32,
    data: T,
}

#[derive(PartialEq, Eq, Debug)]
struct Cell<T: Eq> {
    weight: u32,
    data: T,
}

impl<T: Eq> Ord for HeuristicCell<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)

        // self.heuristics
        //     .cmp(&other.heuristics)
        //     .then(self.weight.cmp(&other.weight))
    }
}

impl<T: Eq> PartialOrd for HeuristicCell<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Eq> Ord for Cell<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl<T: Eq> PartialOrd for Cell<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.weight.cmp(&other.weight))
    }
}

fn path_finding(grid: &Vec<Vec<u32>>, start: (u32, u32), end: (u32, u32)) -> Option<u32> {
    let mut seen: HashMap<((u32, u32), Vec<(i32, i32)>), u32> = HashMap::new();
    let mut stack = BinaryHeap::new();

    seen.insert(((0, 0), Vec::new()), 0);
    stack.push(Reverse(Cell {
        weight: 0,
        data: ((0, 0), Vec::new()),
    }));

    while let Some(Reverse(Cell {
        weight: current_dist,
        data: (current_pos, last_moves),
    })) = stack.pop()
    {
        // println!("queue content: {:?}", stack);
        // println!("seen content: {:?}", seen);

        // println!(
        //     "at:{:?} dist from start: {:?} - last_moves: {:?}",
        //     current_pos, current_dist, last_moves
        // );

        if current_pos == end {
            println!("find end with heat loss: {:?}", current_dist);
            grid_print(grid, &start, &last_moves);
            return Some(current_dist);
            // possible_answers.insert(current_dist);
        }

        neighbors(grid, current_pos)
            // Check that we are not turning back
            .filter(|(neigh_x, neigh_y)| {
                let dir: (i32, i32) = (
                    *neigh_x as i32 - current_pos.0 as i32,
                    *neigh_y as i32 - current_pos.1 as i32,
                );

                match last_moves.last() {
                    Some(prev_dir) => dir != reverse_dir(prev_dir),
                    None => true,
                }
            })
            // Ensure four moves in the same direction
            .filter(|(neigh_x, neigh_y)| {
                // Dir to take to reach this neighbor
                // Dir to take to reach this neighbor
                let dir: (i32, i32) = (
                    *neigh_x as i32 - current_pos.0 as i32,
                    *neigh_y as i32 - current_pos.1 as i32,
                );

                let last_move_opt = last_moves.last();
                if last_move_opt.is_none() {
                    return true;
                } else {
                    let last_move = last_move_opt.unwrap();
                    if dir == *last_move {
                        return last_moves.len() < 3
                            || !last_moves
                                .iter()
                                .rev()
                                .take(3)
                                .all(|previous_dir| previous_dir == &dir);
                    } else {
                        return true;
                    }
                }
            })
            .for_each(|(neigh_x, neigh_y)| {
                let dir: (i32, i32) = (
                    neigh_x as i32 - current_pos.0 as i32,
                    neigh_y as i32 - current_pos.1 as i32,
                );

                // println!("Check neighbour: {:?} dir => {:?}", (neigh_x, neigh_y), dir);
                let mut moves_to_neigh: Vec<(i32, i32)> = last_moves.clone();
                moves_to_neigh.push(dir);

                let dist_to_neigh = current_dist + grid[neigh_y as usize][neigh_x as usize];

                // Max without turning
                let dir_keys = moves_to_neigh
                    .clone()
                    .into_iter()
                    .rev()
                    .take(10)
                    .rev()
                    .collect_vec();

                // let moves_to_neigh = moves_to_neigh.clone().into_iter().rev().take(3).rev().collect_vec();

                match seen.get(&((neigh_x, neigh_y), dir_keys.clone())) {
                    Some(dist) if dist > &dist_to_neigh => {
                        seen.insert(((neigh_x, neigh_y), dir_keys), dist_to_neigh);
                        stack.push(Reverse(Cell {
                            weight: dist_to_neigh,
                            data: ((neigh_x, neigh_y), moves_to_neigh),
                        }));
                    }
                    None => {
                        seen.insert(((neigh_x, neigh_y), dir_keys), dist_to_neigh);
                        stack.push(Reverse(Cell {
                            weight: dist_to_neigh,
                            data: ((neigh_x, neigh_y), moves_to_neigh),
                        }));
                    }
                    _other_cases => {} // { println!("{_other_cases:?}") }, // already seen and not interesting
                }
                // println!("{:?}", stack);
            });
    }

    None
}

fn path_finding_ultra_crucible(
    grid: &Vec<Vec<u32>>,
    start: (u32, u32),
    end: (u32, u32),
) -> Option<u32> {
    let mut seen: HashMap<((u32, u32), Vec<(i32, i32)>), u32> = HashMap::new();
    let mut stack = BinaryHeap::new();

    seen.insert(((0, 0), Vec::new()), 0);
    stack.push(Reverse(Cell {
        weight: 0,
        data: ((0, 0), Vec::new()),
    }));

    while let Some(Reverse(Cell {
        weight: current_dist,
        data: (current_pos, last_moves),
    })) = stack.pop()
    {
        // println!("queue content: {:?}", stack);
        // println!("seen content: {:?}", seen);

        // println!(
        //     "at:{:?} dist from start: {:?} - last_moves: {:?}",
        //     current_pos, current_dist, last_moves
        // );

        if current_pos == end
            && last_moves
                .iter()
                .rev()
                .take_while(|dir| dir == &last_moves.last().unwrap())
                .count()
                >= 4
        {
            println!("find end with heat loss: {:?}", current_dist);
            grid_print(grid, &start, &last_moves);
            return Some(current_dist);
            // possible_answers.insert(current_dist);
        }

        neighbors(grid, current_pos)
            // Check that we are not turning back
            .filter(|(neigh_x, neigh_y)| {
                let dir: (i32, i32) = (
                    *neigh_x as i32 - current_pos.0 as i32,
                    *neigh_y as i32 - current_pos.1 as i32,
                );

                match last_moves.last() {
                    Some(prev_dir) => dir != reverse_dir(prev_dir),
                    None => true,
                }
            })
            // Ensure four moves in the same direction
            .filter(|(neigh_x, neigh_y)| {
                // Dir to take to reach this neighbor
                // Dir to take to reach this neighbor
                let dir: (i32, i32) = (
                    *neigh_x as i32 - current_pos.0 as i32,
                    *neigh_y as i32 - current_pos.1 as i32,
                );

                let last_move_opt = last_moves.last();
                if last_move_opt.is_none() {
                    return true;
                } else {
                    let last_move = last_move_opt.unwrap();
                    if dir == *last_move {
                        return last_moves.len() < 10
                            || !last_moves
                                .iter()
                                .rev()
                                .take(10)
                                .all(|previous_dir| previous_dir == &dir);
                    } else {
                        let nb_move_in_dir = last_moves
                            .iter()
                            .rev()
                            .take_while(|prev_dir| prev_dir == &last_move)
                            .take(4)
                            .count();

                        // println!(
                        //     "filter: {:?}: {dir:?} => {:?}, last: {:?}: nb found: {} is {}",
                        //     (neigh_x, neigh_y),
                        //     last_moves.last(),
                        //     last_moves.iter().rev().take(10).rev().collect_vec(),
                        //     nb_move_in_dir,
                        //     nb_move_in_dir >= 4
                        // );

                        return nb_move_in_dir >= 4;
                    }
                }
            })
            .for_each(|(neigh_x, neigh_y)| {
                let dir: (i32, i32) = (
                    neigh_x as i32 - current_pos.0 as i32,
                    neigh_y as i32 - current_pos.1 as i32,
                );

                // println!("Check neighbour: {:?} dir => {:?}", (neigh_x, neigh_y), dir);
                let mut moves_to_neigh: Vec<(i32, i32)> = last_moves.clone();

                let dist_to_neigh = current_dist + grid[neigh_y as usize][neigh_x as usize];
                moves_to_neigh.push(dir);

                // Max without turning
                let dir_keys = moves_to_neigh
                    .clone()
                    .into_iter()
                    .rev()
                    .take(10)
                    .rev()
                    .collect_vec();

                // let moves_to_neigh = moves_to_neigh.clone().into_iter().rev().take(3).rev().collect_vec();

                match seen.get(&((neigh_x, neigh_y), dir_keys.clone())) {
                    Some(dist) if dist > &dist_to_neigh => {
                        seen.insert(((neigh_x, neigh_y), dir_keys), dist_to_neigh);
                        stack.push(Reverse(Cell {
                            weight: dist_to_neigh,
                            data: ((neigh_x, neigh_y), moves_to_neigh),
                        }));
                    }
                    None => {
                        seen.insert(((neigh_x, neigh_y), dir_keys), dist_to_neigh);
                        stack.push(Reverse(Cell {
                            weight: dist_to_neigh,
                            data: ((neigh_x, neigh_y), moves_to_neigh),
                        }));
                    }
                    _other_cases => {} // { println!("{_other_cases:?}") }, // already seen and not interesting
                }
                // println!("{:?}", stack);
            });
    }

    None
}

fn main() -> std::io::Result<()> {
    /* part 1 */
    let grid: Vec<Vec<u32>> = BufReader::new(File::open("input")?)
        .lines()
        .map(|l| l.unwrap().chars().filter_map(|c| c.to_digit(10)).collect())
        .collect::<Vec<Vec<_>>>();

    // println!("grid: {grid:?}");

    let res = path_finding(
        &grid,
        (0, 0),
        (grid[0].len() as u32 - 1, grid.len() as u32 - 1),
    )
    .unwrap();

    println!("p1: {}", res);
    let p2 = path_finding_ultra_crucible(
        &grid,
        (0, 0),
        (grid[0].len() as u32 - 1, grid.len() as u32 - 1),
    )
    .unwrap();
    println!("p2: {}", p2);

    Ok(())
}
