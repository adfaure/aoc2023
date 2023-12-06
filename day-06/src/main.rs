use itertools::Itertools;
use std::io::BufRead;

use std::{fs::File, io::BufReader};

fn main() -> std::io::Result<()> {
    let (times_line, distances_line) = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|line| line.ok())
        .collect_tuple()
        .unwrap();

    let times = times_line
        .split(" ")
        .filter_map(|s| s.parse::<u32>().ok())
        .collect_vec();
    let distance_records = distances_line
        .split(" ")
        .filter_map(|s| s.parse::<u32>().ok())
        .collect_vec();

    let score: u32 = times
        .iter()
        .zip(distance_records.clone())
        .map(|(time, dist)| {
            (0..=*time)
                .map(|press_time| press_time * (time - press_time))
                .filter(|score| *score > dist)
                .count() as u32
        })
        .product();

    println!("p1: {score}");

    let (times_line, distances_line) = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| str::replace(&line, " ", ""))
        .collect_tuple()
        .unwrap();

    let time = times_line
        .split(":")
        .find_map(|s| s.parse::<u64>().ok())
        .unwrap();

    let distance_record = distances_line
        .split(":")
        .find_map(|s| s.parse::<u64>().ok())
        .unwrap();

    let nb_way_to_beat_record = (0..=time)
        .map(|press_time| press_time * (time - press_time))
        .filter(|score| *score > distance_record)
        .count() as u64;

    println!("p2: {nb_way_to_beat_record}");
    Ok(())
}
