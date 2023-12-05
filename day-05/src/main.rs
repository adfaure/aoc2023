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

    let binding = seeds
        .iter()
        .batching(|it| match it.next() {
            None => None,
            Some(x) => match it.next() {
                None => None,
                Some(y) => Some((*x, *y)),
            },
        })
        .collect::<Vec<(i64, i64)>>();

    let r = binding
        .iter()
        .map(|(start, size)| *start..*start + *size)
        .map(|upper_seed_range| {
            maps.iter().fold(
                [upper_seed_range.clone()].to_vec(),
                |ranges, (source_to_dest, stage_mappings)| {
                    println!(
                        "\n\nstage: {source_to_dest:?} - current ranges to check: {:?}",
                        ranges
                    );

                    let (mut a, mut b) = stage_mappings
                        .iter()
                        .map(|(destination, source, range)| {
                            (
                                *destination..(*destination + *range),
                                *source..*source + *range,
                            )
                        })
                        .inspect(|(dest, src)| println!("next from {:?} => {:?}", dest, src))
                        .fold(
                            (ranges.clone(), Vec::new()),
                            |(mut todo, mut refined_ranges), (dest, source)| {
                                println!(
                                    "not matched: {todo:?} already matched: {refined_ranges:?}"
                                );

                                let mut remaining = Vec::new();
                                todo.clone().iter().for_each(|seed_range| {
                                    let (mut not_matched, mut matched) =
                                        combine(&seed_range, &source, &dest);

                                    remaining.append(&mut not_matched);
                                    refined_ranges.append(&mut matched);
                                });

                                (remaining, refined_ranges)
                            },
                        );

                    a.append(&mut b);
                    a
                },
            )
        })
        .flat_map(|ranges| ranges)
        .map(|range| range.start)
        .min();

    println!("p2: {:?}", r);

    Ok(())
}

type R = std::ops::Range<i64>;

fn combine(seed_range: &R, source: &R, dest: &R) -> (Vec<R>, Vec<R>) {
    println!("    range {seed_range:?} | {source:?} -> {dest:?}");

    let mut splitted_ranges = Vec::new();
    let mut todo = Vec::new();
    // src :                |---------|
    // seed: |---------|
    // OR
    // src : |---------|
    // seed:                |---------|
    if seed_range.end < source.start || seed_range.start > source.end {
        println!("\tcase 1 {seed_range:?}: {source:?} -> {dest:?}");
        todo = [seed_range.clone()].to_vec();
    }
    // seed:     |---------|
    // src : |-----------------|
    else if seed_range.start >= source.start && seed_range.end <= source.end {
        println!("\tcase 2 {seed_range:?}: {source:?} -> {dest:?}");
        let match_start_at = seed_range.start - source.start;
        splitted_ranges.push(
            dest.start + match_start_at
                ..dest.start + match_start_at + (seed_range.end - seed_range.start),
        );
    }
    // seed: |-----------------|
    // src :     |---------|
    else if seed_range.start < source.start && seed_range.end > source.end {
        println!("\tcase 3 {seed_range:?}: {source:?} -> {dest:?}");
        // beginning is unchanged
        let beginning = seed_range.start..source.start;
        let matching = dest.clone();
        let end = source.end..seed_range.end;

        todo.push(beginning);
        todo.push(end);
        splitted_ranges.push(matching);
    }
    // seed:     |-----------------|
    // src : |---------|
    else if seed_range.start < source.end && seed_range.end > source.end {
        println!("\tcase 4 {seed_range:?}: {source:?} -> {dest:?}");
        let matching = dest.start + (seed_range.start - source.start)..dest.end;
        let end = source.end..seed_range.end;
        todo.push(end);
        splitted_ranges.push(matching);
    }
    // seed:     |-----------------|
    // src :                  |---------|
    else if seed_range.start < source.start && seed_range.end <= source.end {
        println!("\tcase 5 {seed_range:?}: {source:?} -> {dest:?}");
        let beginning = seed_range.start..source.start;
        let matching = dest.start
            ..dest.start + ((seed_range.end - source.start) - (seed_range.start - source.start))
                - (source.start - seed_range.start);
        todo.push(beginning);
        splitted_ranges.push(matching);
    } else {
        panic!();
    }

    println!("\tresult of splitting: {todo:?} --- {splitted_ranges:?}");
    (todo, splitted_ranges)
}
