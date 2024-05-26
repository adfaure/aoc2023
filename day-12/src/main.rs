use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;
use std::io::BufRead;
use std::iter;
use std::{fs::File, io::BufReader};

fn recurse(
    consecutive_errors_before: u64,
    hotsprings: Vec<char>,
    rem_failures: Vec<u64>,
    // for debug
    before: Vec<char>,
    memo: &mut HashMap<(Vec<char>, Vec<u64>, u64), u64>,
) -> u64 {
    let str = hotsprings.iter().join("");
    let before_str = before.iter().join("");

    match memo.get(&(
        hotsprings.clone(),
        rem_failures.clone(),
        consecutive_errors_before,
    )) {
        Some(value) => {
            // println!(
            //     "cache hit match {:?} -> {value:?} -- {before_str}|{str}",
            //     (
            //         hotsprings.clone().iter().join(""),
            //         rem_failures.clone(),
            //         consecutive_errors_before
            //     )
            // );
            return *value;
        }
        None => {}
    };

    // println!("recurse: {consecutive_errors_before} {before_str}|{str} {rem_failures:?}");
    let mut new_before = before.clone();

    let mut idx = 0;
    let mut new_failures = rem_failures.clone();
    let mut consecutive_errors = consecutive_errors_before;
    let mut not_possible = false;
    'o: for c in &hotsprings {
        if *c == '#' {
            consecutive_errors = consecutive_errors + 1;
        } else if *c == '.' {
            match new_failures.first() {
                Some(n) if *n == consecutive_errors => {
                    // println!("popping {n}");
                    new_failures = new_failures.into_iter().skip(1).collect_vec();
                }
                Some(n) if (*n != consecutive_errors) && consecutive_errors > 0 => {
                    // println!("number of failure is {n} but stops at {consecutive_errors}");
                    not_possible = true;
                }
                Some(_) => {
                    // println!("no error yet, anything can happens: {consecutive_errors}");
                }
                None => {
                    if consecutive_errors > 0 {
                        // println!("consecutive_errors up but {consecutive_errors_before} {new_failures:?}");
                        not_possible = true;
                    }
                    // No more failures to expect
                    // consecutive_errors = 0;
                    // break 'o;
                }
            }
            consecutive_errors = 0;
        } else if *c == '?' {
            break 'o;
        }
        new_before.push(*c);
        idx = idx + 1;
    }

    let remaining_hotsprings = hotsprings.clone().into_iter().skip(idx).collect_vec();

    if not_possible {
        // println!(
        //     "caching impossible {:?} -> 0 --- {before_str}|{str}",
        //     (
        //         hotsprings.clone().iter().join(""),
        //         rem_failures.clone(),
        //         consecutive_errors_before
        //     )
        // );
        memo.insert(
            (
                hotsprings.clone(),
                rem_failures.clone(),
                consecutive_errors_before,
            ),
            0,
        );
        return 0;
    }

    let debug = remaining_hotsprings.iter().join("");
    let d_b = new_before.iter().join("");

    // println!("will check all cases: {d_b}|{debug} -> {new_failures:?}");

    let result = if idx == hotsprings.len() {
        if new_failures.is_empty() {
            // println!("match {d_b}|{debug} -> {new_failures:?}");
            1
        } else {
            0
        }
    } else {
        let mut add_hotsprint = remaining_hotsprings.clone();
        add_hotsprint[0] = '.';
        let f = recurse(
            consecutive_errors,
            add_hotsprint,
            new_failures.clone(),
            new_before.clone(),
            memo,
        );

        // let d = if consecutive_errors >= *first {
        let mut add_failure = remaining_hotsprings.clone();
        add_failure[0] = '#';
        // println!("d calling with {:?}", (consecutive_errors, add_failure.iter().join(""), new_failures.clone(), new_before.clone()));
        let d = recurse(
            consecutive_errors,
            add_failure.clone(),
            new_failures.clone(),
            new_before.clone(),
            memo,
        );
        // println!("d called with {:?}", (consecutive_errors, add_failure.iter().join(""), new_failures.clone(), new_before.clone()));
        // } else {
        //    0
        // };
        // println!("d: {}, f {}", d, f);
        d + f
    };

    // println!(
    //     "caching end of function {:?} -> {result} --  --- {before_str}|{str}",
    //     (
    //         hotsprings.clone().iter().join(""),
    //         rem_failures.clone(),
    //         consecutive_errors_before
    //     )
    // );
    memo.insert(
        (
            hotsprings.clone(),
            rem_failures.clone(),
            consecutive_errors_before,
        ),
        result,
    );
    return result;
}

fn main() -> std::io::Result<()> {
    if true {
        let problems: Vec<_> = BufReader::new(File::open("input")?)
            .lines()
            .filter_map(|line| line.ok())
            .collect();

        let rp1 = 5;
        let solution: u64 = problems
            .par_iter()
            .map(|line| {
                let (hotspring_str, failures_str) = line.split_once(" ").unwrap();
                let hotsrpings =
                    iter::repeat(hotspring_str.chars().chain(iter::repeat('?').take(1)))
                        .take(rp1 - 1)
                        // Insert a . at the end
                        .chain(iter::repeat(
                            hotspring_str.chars().chain(iter::repeat('.').take(1)),
                        ))
                        .take(rp1)
                        .flatten()
                        .map(|c| c)
                        .collect_vec();

                let nb_failures = iter::repeat(failures_str.split(","))
                    .take(rp1)
                    .flatten()
                    .filter_map(|n| n.parse::<u64>().ok())
                    .collect_vec();

                (hotsrpings, nb_failures)
            })
            .map(|(hotsprings, failures)| {
                (
                    hotsprings.clone(),
                    recurse(
                        0,
                        hotsprings,
                        failures.clone(),
                        Vec::new(),
                        &mut HashMap::new(),
                    ),
                )
            })
            // .inspect(|(h, s)| println!("{:?} -> {}", h.iter().join(""), s))
            .map(|(_, s)| s)
            .sum();

        println!("p1: {solution:?}");
    }

    Ok(())
}

// println!("is solution: {}", is_solution);
