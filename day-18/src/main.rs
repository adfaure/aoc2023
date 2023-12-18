use itertools::Itertools;
use regex::Regex;
use std::io::BufRead;
use std::{fs::File, io::BufReader};

fn solve(trenches: Vec<(char, i64)>) -> i64 {
    let (_, edges, _, (min_x, max_x, min_y, max_y)) = trenches.iter().fold(
        ((0, 0), Vec::new(), (0, 0), (0, 0, 0, 0)),
        |mut acc, (d, n)| {
            let cur_pos;
            let (mut cur_size_x, mut cur_size_y) = acc.2;
            let (mut min_x, mut max_x, mut min_y, mut max_y) = acc.3;

            match d {
                'R' => {
                    cur_pos = (acc.0 .0 + n, acc.0 .1);
                    cur_size_x += n;
                    max_x = max_x.max(cur_size_x);
                }
                'L' => {
                    cur_pos = (acc.0 .0 - n, acc.0 .1);
                    cur_size_x -= n;
                    min_x = min_x.min(cur_size_x);
                }
                'D' => {
                    cur_pos = (acc.0 .0, acc.0 .1 + n);
                    cur_size_y += n;
                    max_y = max_y.max(cur_size_y);
                }
                'U' => {
                    cur_pos = (acc.0 .0, acc.0 .1 - n);
                    cur_size_y -= n;
                    min_y = min_y.min(cur_size_y);
                }
                _ => panic!(),
            }

            acc.1.push((acc.0, cur_pos));
            (
                cur_pos,
                acc.1,
                (cur_size_x, cur_size_y),
                (min_x, max_x, min_y, max_y),
            )
        },
    );

    let size_x = max_x - min_x;
    let size_y = max_y - min_y;
    println!(
        "map size: {:?} {:?} {:?}",
        (min_x, max_x),
        (min_y, max_y),
        (size_x, size_y)
    );

    let mut stack = Vec::new();
    // let mut seen = HashSet::new();

    // let mut rng = rand::thread_rng();

    let start = (0, 0);
    stack.push(start);

    // while let Some(current) = stack.pop() {
    //     let mut exit_current = false;
    //     for (neigh_x, neigh_y) in [(0, 1), (0, -1), (1, 0), (-1, 0)].into_iter() {
    //         let (x, y) = (current.0 - neigh_x, current.1 - neigh_y);
    //         let on_edge = edges.iter().any(|((from_x, from_y), (to_x, to_y))| {
    //             let (from, to);
    //             let pos_to_check;

    //             if from_x == to_x && x == *from_x {
    //                 from = *from_y.min(to_y);
    //                 to = *from_y.max(to_y);
    //                 pos_to_check = y;
    //             } else if from_y == to_y && y == *from_y {
    //                 from = *from_x.min(to_x);
    //                 to = *from_x.max(to_x);
    //                 pos_to_check = x;
    //             } else {
    //                 return false;
    //             }
    //             let contains = (from..=to).contains(&pos_to_check);
    //             contains
    //         });

    //         if !on_edge && (x >= max_x || y >= max_y || x < min_x || y < min_y) {
    //             exit_current = true;
    //             break;
    //         }

    //         // println!("{:?} => {:?}", (x, y), on_edge);

    //         if !on_edge && seen.insert((x, y)) {
    //             stack.push((x, y));
    //         }
    //     }

    //     if exit_current {
    //         seen.clear();
    //         stack.clear();

    //         let new_start = (rng.gen::<i64>() % size_x, rng.gen::<i64>() % size_y);

    //         println!("cleared: {new_start:?}");
    //         stack.push(new_start)
    //     }
    // }

    let trenches_size = trenches.iter().fold(0, |acc, (_, n)| acc + n);

    //   println!(
    //       "p1: {:?}: {:?} + {:?}",
    //       seen.len() + trenches_size as usize,
    //       seen.len(),
    //       trenches_size
    //   );

    println!("shoe: {:?}", shoelace(&edges, trenches_size) + 2);
    shoelace(&edges, trenches_size) + 2

    // return seen.len() as i64 + trenches_size
}

fn shoelace(edges: &Vec<((i64, i64), (i64, i64))>, size: i64) -> i64 {
    let mut ps = edges.into_iter().map(|(p, _)| *p).collect_vec();
    ps.push(edges.last().unwrap().1);
    // println!("{edges:?} === {ps:?}");

    let mut res = 0.0;
    for (e1, e2, e3) in ps.iter().tuple_windows() {
        // println!("{:?}", (e1, e2, e3));

        res += (e2.1 * (e1.0 - e3.0)) as f64;
    }

    (res / 2.) as i64 + (size / 2) - 1
}

fn main() -> std::io::Result<()> {
    /* part 1 */
    let r = Regex::new(r"(.) (\d+) (\(#......\))$").unwrap();

    let (trenchs, trenchs_p2) = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| r.is_match(&line))
        .map(|line| {
            let (mut d, n, mut c) = r
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

            // one )
            c.pop();

            let int_dir = c.pop();
            let size_p2 = i64::from_str_radix(&c.chars().skip(2).collect::<String>(), 16).unwrap();

            let d_p2 = match int_dir.unwrap() {
                '0' => 'R',
                '1' => 'D',
                '2' => 'L',
                '3' => 'U',
                _ => panic!(),
            };

            (
                (d.pop().unwrap(), n.parse::<i64>().unwrap()), // p1
                (d_p2, size_p2),
            )
        })
        .fold((vec![], vec![]), |(mut v_p1, mut v_p2), (p1, p2)| {
            v_p1.push(p1);
            v_p2.push(p2);
            (v_p1, v_p2)
        });

    let p1 = solve(trenchs);
    println!("p1: {p1}");

    let p2 = solve(trenchs_p2);
    println!("p2: {p2}");

    Ok(())
}
