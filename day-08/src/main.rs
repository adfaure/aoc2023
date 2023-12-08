use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::collections::HashSet;
use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;
use std::{fs::File, io::BufReader};

fn main() -> std::io::Result<()> {
    let r = Regex::new(r"(...) = \((...), (...)\)").unwrap();

    // P1 doesn't pass on second example
    // let tree = BufReader::new(File::open("input")?)
    //     .lines()
    //     .skip(2)
    //     .filter_map(|line| line.ok())
    //     .map(|line| {
    //         let (n, l, r) = r
    //             .captures_iter(&line)
    //             .map(|capture| {
    //                 capture
    //                     .iter()
    //                     .skip(1)
    //                     .filter_map(|m| m)
    //                     .map(|m| m.as_str())
    //                     .collect_vec()
    //             })
    //             .flatten()
    //             .map(String::from)
    //             .collect_tuple()
    //             .unwrap();
    //         (n, (l, r))
    //     })
    //     .collect::<HashMap<String, (String, String)>>();

    // let mut current_node = String::from("AAA");

    // let directions = BufReader::new(File::open("input")?)
    //     .lines()
    //     .find_map(|line| line.ok())
    //     .unwrap()
    //     .chars()
    //     .cycle()
    //     .enumerate()
    //     .find_map(|(total_iteration, direction)| {
    //         println!("current: {current_node}");
    //         let (l, r) = tree.get(&current_node).unwrap();

    //         current_node = match direction {
    //             'L' => String::from(l),
    //             'R' => String::from(r),
    //             _ => unreachable!(),
    //         };

    //         if current_node == "ZZZ" {
    //             return Some(total_iteration + 1);
    //         } else {
    //             return None;
    //         }
    //     });

    // println!("p1: {directions:?}");

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

    let mut assert_cycle = HashSet::new();
    let directions = BufReader::new(File::open("input")?)
        .lines()
        .find_map(|line| line.ok())
        .unwrap()
        .chars()
        .cycle()
        .enumerate()
        .fold_while(
            // Added 0 bc I need the return type to contain the enumerate
            (starts, 0),
            |(current_nodes, _), (total_iteration, direction)| {
                let new_nodes = current_nodes.iter().fold(
                    (0, Vec::new()),
                    |(end_reached, mut nodes), current_node| {
                        let (l, r) = tree.get(current_node).unwrap();
                        let new_node = match direction {
                            'L' => String::from(l),
                            'R' => String::from(r),
                            _ => unreachable!(),
                        };
                        nodes.push(new_node.clone());

                        if new_node.chars().last().unwrap() == 'Z' {
                            return (end_reached + 1, nodes);
                        }

                        (end_reached, nodes)
                    },
                );

                if new_nodes.0 == current_nodes.len() {
                    return Done((new_nodes.1, total_iteration + 1));
                }

                let all = new_nodes.1.join("");
                assert!(assert_cycle.insert(all.clone()), "We hit a cycle: {all:?}");

                Continue((new_nodes.1, 0))
            },
        );

    println!("{directions:?}");
    Ok(())
}
