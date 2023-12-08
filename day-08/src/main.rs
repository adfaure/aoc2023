use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::BufRead;
use std::{fs::File, io::BufReader};

fn main() -> std::io::Result<()> {
    let r = Regex::new(r"(...) = \((...), (...)\)").unwrap();

    // P1 doesn't pass on second example
    let tree = BufReader::new(File::open("input")?)
        .lines()
        .skip(2)
        .filter_map(|line| line.ok())
        .map(|line| {
            let (n, l, r) = r
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
            (n, (l, r))
        })
        .collect::<HashMap<String, (String, String)>>();

    let mut current_node = String::from("AAA");

    let directions = BufReader::new(File::open("input")?)
        .lines()
        .find_map(|line| line.ok())
        .unwrap()
        .chars()
        .cycle()
        .enumerate()
        .find_map(|(total_iteration, direction)| {
            // println!("current: {current_node}");
            let (l, r) = tree.get(&current_node).unwrap();

            current_node = match direction {
                'L' => String::from(l),
                'R' => String::from(r),
                _ => unreachable!(),
            };

            if current_node == "ZZZ" {
                return Some(total_iteration + 1);
            } else {
                return None;
            }
        });

    println!("p1: {directions:?}");

    let (starts, tree) = BufReader::new(File::open("input")?)
        .lines()
        .skip(2)
        .filter_map(|line| line.ok())
        .map(|line| {
            let (n, l, r) = r
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
            (n, (l, r))
        })
        .fold(
            (
                Vec::new(),
                HashMap::new() as HashMap<String, (String, String)>,
            ),
            |(mut starting_nodes, mut tree), (node, (l, r))| {
                if node.chars().nth(2).unwrap() == 'A' {
                    starting_nodes.push(node.clone());
                }
                tree.insert(node, (l, r));

                (starting_nodes, tree)
            },
        );

    let directions = BufReader::new(File::open("input")?)
        .lines()
        .find_map(|line| line.ok())
        .unwrap();

    let direction_size = directions.len();

    let r = starts
        .into_iter()
        .map(|node| {
            let mut seen = HashSet::new();
            let mut dist_from_start: HashMap<(String, usize), usize> = HashMap::new();

            let mut nb_possible_ending_found = 0; // Just to check
            let mut iterection = directions.chars().cycle().enumerate();

            let initial_dir = iterection.next().unwrap();
            // println!("starts: {:?}", initial_dir);

            let mut current_node: (String, char, usize) = (node, initial_dir.1, initial_dir.0);
            dist_from_start.insert((current_node.0.clone(), initial_dir.0), 0 as usize);

            while seen.insert(current_node.clone()) {
                if current_node.0.chars().last().unwrap() == 'Z' {
                    nb_possible_ending_found += 1;
                }

                // println!("Inserting: {:?}", current_node);
                let (l, r) = tree.get(&current_node.0).unwrap();
                let direction = iterection.next().unwrap();

                dist_from_start
                    .entry((current_node.0.clone(), (direction.0 - 1) % direction_size))
                    .or_insert(direction.0 - 1);

                current_node.0 = match current_node.1 {
                    'L' => String::from(l),
                    'R' => String::from(r),
                    _ => unreachable!(),
                };
                current_node.1 = direction.1;
                current_node.2 = direction.0 % direction_size;

                // println!("current at end fo loop: {:?}", current_node);
            }

            assert!(
                nb_possible_ending_found == 1,
                "More endings than expected {nb_possible_ending_found}"
            );

            let end_cycle: usize = iterection.next().unwrap().0 - 1;
            let cycle_start: (String, usize) = (
                current_node.0.clone(),
                *dist_from_start
                    .get(&(current_node.0.clone(), current_node.2 % direction_size))
                    .unwrap(),
            );
            let ending = dist_from_start
                .iter()
                .find(|(k, _)| k.0.chars().last().unwrap() == 'Z')
                .unwrap()
                .1;
            let cycle_size = end_cycle - cycle_start.1;
            (*ending, cycle_size)
        })
        .map(|(f, _)| f as u64)
        .reduce(num::integer::lcm);

    println!("p2: {r:?}");
    Ok(())
}

// R
//
// AAA = (BBB, CCC)
// EEA = (UUU, UUU)
// UUU = (UUU, VVV)
// VVV = (UUU, YYY)
// YYY = (UUU, TTT)
// TTT = (UUU, DDD)
// BBB = (XXX, DDD)
// CCC = (DDD, DDD)
// DDD = (EEE, EEE)
// EEE = (ZZZ, GGG)
// GGG = (ZZZ, ZZZ)
// ZZZ = (DDD, DDD)
// XXX = (XXX, XXX)
