use itertools::Itertools;
use regex::Regex;
use std::io::BufRead;

use std::{fs::File, io::BufReader};

fn main() -> std::io::Result<()> {

    let re = Regex::new(r"\d+").unwrap();

    /* part 1 */
    let grid: Vec<Vec<String>> = BufReader::new(File::open("input")?)
        .lines()
        .map(|l| l.unwrap().chars().map(|c| c.to_string()).collect())
        .collect::<Vec<Vec<_>>>();

    let sum: i32 = BufReader::new(File::open("input")?)
        .lines()
        .enumerate()
        .flat_map(|(n, l)| {
            let line = l.unwrap().to_string();
            let n = n as i32;

            return re
                .captures_iter(&line)
                .filter_map(|capture| {
                    for i in capture.get(0).unwrap().range() {
                        let i = i as i32;
                        /* Look neighbors */
                        for x in i - 1..i + 2 {
                            for y in n - 1..n + 2 {
                                if x >= 0
                                    && y >= 0
                                    && x < grid[0].len() as i32
                                    && y < grid.len() as i32
                                {
                                    if grid[y as usize][x as usize]
                                        .chars()
                                        .all(|c| c.is_ascii() && !c.is_ascii_digit() && c != '.')
                                    {
                                        return capture
                                            .get(0)
                                            .unwrap()
                                            .as_str()
                                            .parse::<i32>()
                                            .ok();
                                    }
                                }
                            }
                        }
                    }
                    None
                })
                .collect::<Vec<_>>();
        })
        .sum();

    println!("p1: {sum:?}");

    let mut res = BufReader::new(File::open("input")?)
        .lines()
        .enumerate()
        .flat_map(|(n, l)| {
            let line = l.unwrap().to_string();
            let n = n as i32;

            return re
                .captures_iter(&line)
                .filter_map(|capture| {
                    for i in capture.get(0).unwrap().range() {
                        let i = i as i32;
                        /* Look neighbors */
                        for x in i - 1..i + 2 {
                            for y in n - 1..n + 2 {
                                if x >= 0
                                    && y >= 0
                                    && x < grid[0].len() as i32
                                    && y < grid.len() as i32
                                {
                                    if grid[y as usize][x as usize].chars().all(|c| c == '*') {
                                        return Some((
                                            (x, y),
                                            capture
                                                .get(0)
                                                .unwrap()
                                                .as_str()
                                                .parse::<i32>()
                                                .unwrap(),
                                        ));
                                    }
                                }
                            }
                        }
                    }
                    None
                })
                .collect::<Vec<_>>();
        })
        .collect::<Vec<_>>();

    res.sort();

    let power = res.iter()
        .group_by(|(pos, _)| pos)
        .into_iter()
        .map(|(pos, number_group)| {
            (
                pos,
                number_group.map(|(_, elem)| *elem).collect::<Vec<i32>>(),
            )
        })
        .map(|(_, group)| group)
        .filter(|group| group.len() > 1)
        .map(|gears| gears.iter().product::<i32>())
        .sum::<i32>();

    println!("p2: {:?}", power);
    Ok(())
}
