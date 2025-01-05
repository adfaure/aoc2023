use itertools::Itertools;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};


fn print_digraph(links: &HashSet<(String, String)>) {
    println!("digraph {{
      layout=neato
        ");
    for link in links {
        println!("\t{} -> {}", link.0, link.1);
    }

    println!("}}");
}

fn main() -> std::io::Result<()> {
    // let graph = HashMap::new();

    let mut links = BufReader::new(File::open("input")?)
        .lines()
        .map_while(|l| l.ok())
        .map(|line| {
            line.split(':')
                .map(|s| s.to_string())
                .collect_tuple()
                .unwrap()
        })
        .flat_map(|(root, links)| {
            links
                .split_whitespace()
                .map(move |con| {
                    let t = (root.to_string(), con.to_string());
                    match t.0.cmp(&t.1) {
                        Ordering::Less => (t.0, t.1),
                        Ordering::Greater | Ordering::Equal => (t.1, t.0),
                    }
                })
                .collect_vec()
        })
        .sorted()
        .collect::<HashSet<_>>();

    // println!("links: {:?}", links);
    // Print the graph with graphviz
    // dot d.dot -Tpdf > g.pdf
    // Find the three culprits
    print_digraph(&links);

    // manually remove them
    assert!(links.remove(&("jzv".to_string(), "qvq".to_string())));
    assert!(links.remove(&("gtj".to_string(), "tzj".to_string())));
    assert!(links.remove(&("bbp".to_string(), "dvr".to_string())));

    let groups = find_groups(&links);
    println!("p1: {:?}", groups.into_iter().map(|g| g.len()).product::<usize>());

    // Tried to Brute Force
    // let group = links.iter().combinations(3)
    //     .par_bridge()
    //     .find_map_any(|truple| {
    //         let (a, b, c) = truple.iter().collect_tuple().unwrap();

    //         let mut tweaked = links.clone();
    //         assert!(tweaked.remove(a));
    //         assert!(tweaked.remove(b));
    //         assert!(tweaked.remove(c));

    //         let groups = find_groups(&tweaked);
    //         if groups.len() > 1 {
    //             println!("remove: {:?}", truple);
    //             println!("groups: {:?}", groups);
    //             return Some(groups)
    //         }
    //         None
    //     })
    //     .unwrap();

    // println!("p1: {:?}", group.into_iter().map(|g| g.len()).product::<usize>());

    Ok(())
}

fn find_groups(links: &HashSet<(String, String)>) -> Vec<HashSet<String>> {
    let mut groups: Vec<HashSet<String>> = vec![];

    for (a, b) in links {
        if !groups
            .iter()
            .any(|group| group.contains(a) || group.contains(b))
        {
            groups.push(find_group(links, a, &mut HashSet::new()));
        }
    }

    groups
}

fn find_group(
    links: &HashSet<(String, String)>,
    link: &str,
    group: &mut HashSet<String>,
) -> HashSet<String> {
    // println!("{} -- {:?}", link, group);
    if group.insert(link.to_string()) {
        return links
            .iter()
            .filter(|(a, b)| link == a || link == b)
            .flat_map(|(a, b)| {
                if link == a && !group.contains(b) {
                    find_group(links, b, group)
                } else if !group.contains(a) {
                    find_group(links, a, group)
                } else {
                    group.clone()
                }
            })
            .collect();
    }

    group.clone()
}
