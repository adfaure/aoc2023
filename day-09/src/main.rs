use itertools::Itertools;
use std::io::BufRead;
use std::{fs::File, io::BufReader};

fn main() -> std::io::Result<()> {
    // P1 doesn't pass on second example
    let res = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.split(" ")
                .filter_map(|nb| nb.parse::<i32>().ok())
                .collect_vec()
        })
        .map(|serie| {
            let mut stages = Vec::new();
            let mut stage = serie.clone();
            stages.push(stage.clone());

            while !stage.iter().all(|e| e == &0) {
                stage = stage
                    .iter()
                    .tuple_windows()
                    .map(|(a, b)| b - a)
                    .collect_vec();
                stages.push(stage.clone());
            }

            stages
        })
        .map(|stages| {
            stages.iter().rev().skip(1).fold(0, |acc, stage| {
                let last = stage.last().unwrap();
                last + acc
            })
        })
        .sum::<i32>();

    println!("p1: {res:?}");

    let res = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.split(" ")
                .filter_map(|nb| nb.parse::<i32>().ok())
                .collect_vec()
        })
        .map(|serie| {
            let mut stages = Vec::new();
            let mut stage = serie.clone();
            stages.push(stage.clone());

            while !stage.iter().all(|e| e == &0) {
                stage = stage
                    .iter()
                    .tuple_windows()
                    .map(|(a, b)| b - a)
                    .collect_vec();
                stages.push(stage.clone());
            }

            stages
        })
        .map(|stages| {
            stages.iter().rev().skip(1).fold(0, |acc, stage| {
                let last = stage.first().unwrap();
                last - acc
            })
        })
        .sum::<i32>();

    println!("p1: {res:?}");
    Ok(())
}
