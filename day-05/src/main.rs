use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;
use std::io::BufRead;

use std::{fs::File, io::BufReader};

fn main() -> std::io::Result<()> {
    let input: Vec<String> = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|line| line.ok())
        .collect();

    let seeds = input[0]
        .chars()
        .skip(7)
        .collect::<String>()
        .split(" ")
        .filter_map(|seed| seed.parse::<i64>().ok())
        .collect::<Vec<_>>();

    let maps = input
        .iter()
        .skip(2)
        .batching(|iter| {
            // Parsing maps
            let re = Regex::new(r"(\w+)-to-(\w+) map:$").unwrap();

            let mut ranges: Vec<(i64, i64, i64)> = Vec::new();
            let mut opt = iter.next();

            if opt.is_none() {
                return None;
            }

            let mut from_to = None;

            while let Some(line) = opt {
                if line == "" {
                    break;
                }

                if re.is_match(&line) {
                    let caps = re.captures(line).unwrap();
                    let from = String::from(caps.get(1).unwrap().as_str());
                    let to = String::from(caps.get(2).unwrap().as_str());

                    from_to = Some((from, to))
                } else {
                    ranges.push(
                        line.split(" ")
                            .filter_map(|n| n.parse::<i64>().ok())
                            .collect_tuple()
                            .unwrap(),
                    );
                }

                opt = iter.next();
            }
            return Some((from_to.unwrap(), ranges));
        })
        .collect::<Vec<_>>();

    let r = seeds
        .iter()
        .map(|seed| {
            maps.iter().fold(*seed, |mut n, map| {
                let n_found = match map
                    .1
                    .iter()
                    .find(|ranges| (ranges.1..ranges.1 + ranges.2).contains(&n))
                    .and_then(|matching_range| Some(matching_range.0 + (n - matching_range.1)))
                {
                    Some(corresponding) => corresponding,
                    None => n,
                };
                n = n_found;
                n
            })
        })
        .min();

    println!("p1: {:?}", r);

    let r = seeds
        .iter()
        .batching(|it| match it.next() {
            None => None,
            Some(x) => match it.next() {
                None => None,
                Some(y) => Some((*x, *y)),
            },
        })
        .collect::<Vec<(i64, i64)>>()
        .iter()
        .map(|(start, size)| *start..*start + *size)
        // .flat_map(|(start, range)| *start..*start + *range)
        .flat_map(|upper_seed_range| {
            maps.iter().fold(
                [upper_seed_range.clone()].to_vec(),
                |ranges, (source_to_dest, mapping)| {
                    println!(
                        "\n\nstage: {source_to_dest:?} - current ranges to check: {:?}",
                        ranges
                    );

                    let mut refined_ranges = Vec::new();

                    ranges.iter().for_each(|seed_range| {
                        // loop all different stages (seed to soil, soil to etc)
                        let mut new_ranges = mapping
                            .iter()
                            .map(|(destination, source, range)| {
                                (
                                    *destination..(*destination + *range),
                                    *source..*source + *range,
                                )
                            })
                            .flat_map(|(dest, source)| {
                                println!("{seed_range:?}: {source:?} -> {dest:?}");
                                let mut splitted_ranges = Vec::new();
                                // src :                |---------|
                                // seed: |---------|
                                // OR
                                // src : |---------|
                                // seed:                |---------|
                                if seed_range.end < source.start || seed_range.start > source.end {
                                    println!("case 1 {seed_range:?}: {source:?} -> {dest:?}");
                                }
                                // seed:     |---------|
                                // src : |-----------------|
                                else if seed_range.start >= source.start
                                    && seed_range.end <= source.end
                                {
                                    println!("case 2 {seed_range:?}: {source:?} -> {dest:?}");
                                    let match_start_at = seed_range.start - source.start;
                                    splitted_ranges.push(
                                        dest.start + match_start_at
                                            ..dest.start
                                                + match_start_at
                                                + (seed_range.end - seed_range.start),
                                    );
                                }
                                // seed: |-----------------|
                                // src :     |---------|
                                else if seed_range.start < source.start
                                    && seed_range.end > source.end
                                {
                                    println!("case 3 {seed_range:?}: {source:?} -> {dest:?}");
                                    // beginning is unchanged
                                    let _beginning = seed_range.start..source.start;
                                    let _matching = dest.clone();
                                    let _end = source.end..seed_range.end;
                                    splitted_ranges
                                        .append(&mut [_beginning, _matching, _end].to_vec());
                                }
                                // seed:     |-----------------|
                                // src : |---------|
                                else if seed_range.start < source.end
                                    && seed_range.end > source.end
                                {
                                    println!("case 4 {seed_range:?}: {source:?} -> {dest:?}");
                                    let _matching =
                                        dest.start + (seed_range.start - source.start)..dest.end;
                                    let _end = source.end..seed_range.end;
                                    splitted_ranges.append(&mut [_matching, _end].to_vec());
                                }
                                // seed:     |-----------------|
                                // src :                  |---------|
                                else if seed_range.start < source.start
                                    && seed_range.end < source.end
                                {
                                    println!("case 5 {seed_range:?}: {source:?} -> {dest:?}");
                                    let _beginning = seed_range.start..source.start;
                                    let _matching = dest.start
                                        ..dest.start
                                            + ((seed_range.end - source.start)
                                                - (seed_range.start - source.start))
                                            - (source.start - seed_range.start);
                                    splitted_ranges.append(&mut [_beginning, _matching].to_vec());
                                }

                                assert!(
                                    splitted_ranges.is_empty()
                                        || splitted_ranges
                                            .iter()
                                            .map(|r| r.end - r.start)
                                            .sum::<i64>()
                                            == seed_range.end - seed_range.start
                                );

                                splitted_ranges
                            })
                            .collect_vec();

                        refined_ranges.append(&mut new_ranges);
                    });

                    println!("refined: {:?}", refined_ranges);

                    if refined_ranges.is_empty() {
                        return ranges;
                    } else {
                        return refined_ranges;
                    }
                },
            )
        })
        .map(|range| range.start)
        .min();

    println!("p2: {:?}", r);

    Ok(())
}
