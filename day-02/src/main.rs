use itertools::Itertools;
use std::io::BufRead;

use std::{fs::File, io::BufReader};

fn main() -> std::io::Result<()> {
    let maybe_bag = (12, 13, 14);

    /* part 1 */
    let res = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|l| {
            let content: Vec<String> = l.unwrap().split(":").map(|s| String::from(s)).collect();

            let game_id = content[0]
                .split(" ")
                .filter_map(|game_and_id| game_and_id.parse::<i32>().ok())
                .collect::<Vec<i32>>()[0];

            let r = content[1]
                .split(";")
                .map(|draw| {
                    draw.split(",")
                        .map(|s| {
                            s.trim()
                                .split(" ")
                                .map(|s| String::from(s))
                                .collect_tuple()
                                .unwrap()
                        })
                        .map(|(n, color)| (color, n.parse::<i32>().unwrap()))
                        .fold((0, 0, 0) as (i32, i32, i32), |mut acc, iter| {
                            match iter.0.as_str() {
                                "red" => {
                                    acc.0 = iter.1;
                                }
                                "green" => {
                                    acc.1 = iter.1;
                                }
                                "blue" => {
                                    acc.2 = iter.1;
                                }
                                _ => panic!(),
                            };
                            acc
                        })
                })
                .all(|(red, green, blue)| {
                    red <= maybe_bag.0 && green <= maybe_bag.1 && blue <= maybe_bag.2
                });

            if r {
                Some(game_id)
            } else {
                None
            }
        })
        .sum::<i32>();

    println!("p1: {:?}", res);

    /* part 1 */
    let res = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|l| {
            let content: Vec<String> = l.unwrap().split(":").map(|s| String::from(s)).collect();
            let r = content[1]
                .split(";")
                .map(|draw| {
                    draw.split(",")
                        .map(|s| {
                            s.trim()
                                .split(" ")
                                .map(|s| String::from(s))
                                .collect_tuple()
                                .unwrap()
                        })
                        .map(|(n, color)| (color, n.parse::<i32>().unwrap()))
                        .fold((0, 0, 0) as (i32, i32, i32), |mut acc, iter| {
                            match iter.0.as_str() {
                                "red" => {
                                    acc.0 = iter.1;
                                }
                                "green" => {
                                    acc.1 = iter.1;
                                }
                                "blue" => {
                                    acc.2 = iter.1;
                                }
                                _ => panic!(),
                            };
                            acc
                        })
                })
                .fold((0, 0, 0), |mut acc, (r, g, b)| {
                    if acc.0 < r {
                        acc.0 = r;
                    }

                    if acc.1 < g {
                        acc.1 = g;
                    }

                    if acc.2 < b {
                        acc.2 = b;
                    }

                    acc
                });

            Some(r.0 * r.1 * r.2)
        })
        .sum::<i32>();

    println!("p1: {:?}", res);

    Ok(())
}
