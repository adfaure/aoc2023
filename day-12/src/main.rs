use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::BufRead;
use std::iter;
use std::{fs::File, io::BufReader};

fn check(hotsprings: &Vec<char>, nb_failures: &Vec<u32>) -> bool {
    // println!("try {:?} {:?}", hotsprings.iter().join(""), nb_failures);
    // yuck

    let mut total_checked = 0;
    for (i, (_, group)) in hotsprings
        .iter()
        .group_by(|c| **c == '#')
        .into_iter()
        .filter(|(match_value, _)| *match_value)
        .enumerate()
    {
        total_checked += 1;
        if i >= nb_failures.len() {
            return false;
        } else {
            let failure_group: Vec<_> = group.collect();
            if failure_group.len() as u32 != nb_failures[i] {
                return false;
            }
        }
    }

    // println!("{:?} {:?}", hotsprings.iter().join(""), nb_failures);
    // println!("{} {}", total_checked, nb_failures.len());
    total_checked == nb_failures.len()
}

fn check_allow_failure_overflow(hotsprings: &Vec<char>, nb_failures: &Vec<u32>) -> bool {
    // yuck
    let mut total_checked = 0;
    for (i, (_, group)) in hotsprings
        .iter()
        .group_by(|c| **c == '#')
        .into_iter()
        .filter(|(match_value, _)| *match_value)
        .enumerate()
    {
        total_checked += 1;

        let failure_group: Vec<_> = group.collect();
        if failure_group.len() as u32 != nb_failures[i] {
            return false;
        }
    }

    return true;

    // println!("{:?} {:?}", hotsprings.iter().join(""), nb_failures);
    // println!("{} {}", total_checked, nb_failures.len());
    // total_checked == nb_failures.len()
}

fn recurse(hotsprings: Vec<char>, nb_failures: &Vec<u32>) -> u32 {
    // println!("{:?} {:?}", hotsprings.iter().join(""), nb_failures);
    match hotsprings
        .iter()
        .enumerate()
        .filter(|(_, c)| **c == '?')
        .next()
    {
        Some((next_joker_pos, _)) => {
            let mut add_failure = hotsprings.clone();
            let mut add_hotsprint = hotsprings.clone();

            add_failure[next_joker_pos] = '#';
            add_hotsprint[next_joker_pos] = '.';
            recurse(add_failure, nb_failures) + recurse(add_hotsprint, nb_failures)
        }
        None => {
            if check(&hotsprings, &nb_failures) {
                // println!("p1 yeah: {:?}", hotsprings.iter().join(""));
                return 1;
            } else {
                return 0;
            }
        }
    }
}

fn check_incomplete(hotsprings: &Vec<char>, nb_failures: &Vec<u32>) -> (bool, Vec<u32>) {
    //println!("try {:?} {:?}", hotsprings.iter().join(""), nb_failures);
    let mut total_checked = Vec::new();
    let mut exist = false;

    for (i, (_, group)) in hotsprings
        .iter()
        .enumerate()
        .group_by(|(idx, c)| **c == '#')
        .into_iter()
        .filter(|(match_value, _)| *match_value)
        .enumerate()
    {
        exist = true;
        if i >= nb_failures.len() {
            return (false, Vec::new());
        } else {
            let group_size = nb_failures[i];
            let failure_group: Vec<_> = group.collect();

            let last_char = failure_group.iter().last().unwrap();
            let first_char = failure_group.iter().next().unwrap();

            if last_char.0 + 1 == hotsprings.len() && last_char.1 == &'#' {
                total_checked.push(group_size);
                break;
            }

            if failure_group.is_empty() || failure_group.len() as u32 != group_size {
                if first_char.1 == &'#' && first_char.0 == 0 {
                } else {
                    return (false, Vec::new());
                }
            } else {
                total_checked.push(group_size);
            }
        }
    }

    if !exist {
        return (false, [].to_vec());
    } else {
        return (true, total_checked);
    }

    // println!("{:?} {:?}", hotsprings.iter().join(""), nb_failures);
    // println!("{} {}", total_checked, nb_failures.len());
    // total_checked < nb_failures.len()
}

fn chunk_recurse_p2(
    memo: &mut HashMap<(Vec<char>, Vec<u32>), HashSet<(u32, Vec<u32>)>>,
    hotsprings: Vec<char>,
    nb_failures: &Vec<u32>,
) -> Vec<(u32, Vec<u32>)> {
    if memo.contains_key(&(hotsprings.clone(), nb_failures.to_vec())) {
        // println!(
        //     "I know: {} {:?}",
        //     hotsprings.iter().join(""),
        //     memo.get(&(hotsprings.clone(), nb_failures.to_vec()))
        //         .unwrap()
        // );
        return memo
            .get(&(hotsprings, nb_failures.to_vec()))
            .unwrap()
            .clone()
            .into_iter()
            .collect_vec();
    }

    let res = match hotsprings
        .iter()
        .enumerate()
        .filter(|(_, c)| **c == '?')
        .next()
    {
        Some((next_joker_pos, _)) => {
            let mut add_failure = hotsprings.clone();
            let mut add_hotsprint = hotsprings.clone();

            add_failure[next_joker_pos] = '#';
            add_hotsprint[next_joker_pos] = '.';

            let mut result = Vec::new();
            let mut diese = chunk_recurse_p2(memo, add_failure.clone(), &nb_failures);
            let mut dot = chunk_recurse_p2(memo, add_hotsprint.clone(), &nb_failures);

            result.append(&mut diese);
            result.append(&mut dot);

            result
        }
        None => {
            let (checked, nb_match) = check_incomplete(&hotsprings, &nb_failures);
            if checked {
                //  println!(
                //      "yeah: {:?} {:?}\n{:?}",
                //      hotsprings.iter().join(""),
                //      nb_failures,
                //      nb_match
                //  );
                [(1, nb_match)].to_vec()
            } else {
                [].to_vec()
            }
        }
    };

    memo.insert(
        (hotsprings, nb_failures.to_vec()),
        HashSet::from_iter(res.clone()),
    );

    res
}

fn recurse_p2(hotsprings: Vec<char>, nb_failures: &Vec<u32>, orig_len: u32) -> u32 {
    // println!("{:?} {:?}", hotsprings.iter().join(""), nb_failures);

    let mut memo = HashMap::new();
    let mut solutions = HashSet::new();

    for chunk in hotsprings
        .clone()
        .into_iter()
        .chunks(orig_len as usize)
        .into_iter()
        .take(1)
    {
        let hotspring_chunk = chunk.collect_vec();

        (0..=nb_failures.len()).for_each(|i| {
            let failures_shift = nb_failures
                .clone()
                .into_iter()
                .cycle()
                .skip(i)
                .take(nb_failures.len() + 1)
                .collect_vec();

            let solution = chunk_recurse_p2(&mut memo, hotspring_chunk.clone(), &failures_shift);

            solutions.extend(solution.into_iter());
        });
    }

    let possibilities = memo
        .iter()
        .filter(|(_k, v)| !v.is_empty())
        .filter(|(k, _v)| k.0.iter().all(|c| *c != '?'))
        .map(|(k, v)| (k.0.clone(), v))
        .map(|(k, _v)| k)
        .collect_vec();

    // println!("p: {:?}", possibilities);

    let all_nb_failures = iter::repeat(nb_failures)
        .take(5)
        .flatten()
        .map(|n| *n)
        .collect_vec();

    let res = itertools::repeat_n(possibilities.iter(), 5)
        .multi_cartesian_product()
        .filter(|vv| {
            check(
                &vv.into_iter()
                    .map(|v| v.into_iter())
                    .flatten()
                    .map(|c| *c)
                    .collect_vec(),
                &all_nb_failures,
            )
        })
        .count();

    println!("{:?} : {}", hotsprings.iter().join(""), res);

    res as u32
}

fn main() -> std::io::Result<()> {
    if false {
        let problems: Vec<_> = BufReader::new(File::open("input")?)
            .lines()
            .filter_map(|line| line.ok())
            .collect();

        let rp1 = 5;
        let solution: u32 = problems
            .iter()
            .map(|line| {
                let (hotspring_str, failures_str) = line.split_once(" ").unwrap();
                let hotsrpings = iter::repeat(hotspring_str.chars().chain(['?']))
                    .take(rp1)
                    .flatten()
                    .map(|c| c)
                    .collect_vec();
                let nb_failures = iter::repeat(failures_str.split(","))
                    .take(rp1)
                    .flatten()
                    .filter_map(|n| n.parse::<u32>().ok())
                    .collect_vec();

                (hotsrpings, nb_failures)
            })
            .map(|(hotsprings, failures)| recurse(hotsprings, &failures))
            .sum();

        println!("p1: {solution:?}");
    }
    let repeat = 5;

    let problems: Vec<_> = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|line| line.ok())
        .collect();

    let solution: u32 = problems
        .iter()
        .map(|line| {
            let (hotspring_str, failures_str) = line.split_once(" ").unwrap();
            let hotsrpings = iter::repeat(hotspring_str.chars().chain(iter::repeat('?').take(1)))
                .take(repeat - 1)
                // Insert a . at the end
                .chain(iter::repeat(
                    hotspring_str.chars().chain(iter::repeat('.').take(1)),
                ))
                .take(repeat)
                .flatten()
                .map(|c| c)
                .take((1 + hotspring_str.len()) * repeat)
                .collect_vec();

            println!("created {}", hotsrpings.iter().join(""));

            let nb_failures = failures_str
                .split(",")
                .filter_map(|n| n.parse::<u32>().ok())
                .collect_vec();

            (hotsrpings, nb_failures, hotspring_str.len())
        })
        .map(|(hotsprings, failures, len)| recurse_p2(hotsprings, &failures, 1 + len as u32))
        .sum();

    println!("p2: {solution:?}");

    Ok(())
}

// println!("is solution: {}", is_solution);
