use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;
use std::rc::Rc;
use std::str::FromStr;
use std::{fs::File, io::BufReader};

#[derive(Debug, PartialEq, Eq)]
struct GearPart {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl GearPart {
    fn score(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    FnLess(char, u64, String),
    FnGreater(char, u64, String),
    Label(String),
    Accept,
    Reject,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseInstuctionError;

impl FromStr for Instruction {
    type Err = ParseInstuctionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fn_parse = Regex::new(r"([xmsa])([<>])(\d+):(.*)$").unwrap();
        if fn_parse.is_match(s) {
            let (attr, op, number, out) = fn_parse
                .captures_iter(&s)
                .next()
                .unwrap()
                .extract::<4>()
                .1
                .into_iter()
                .collect_tuple()
                .unwrap();
            let attr = attr.chars().next().unwrap();
            let op = op.chars().next().unwrap();
            let value = number.parse::<u64>().unwrap();
            if op == '>' {
                return Ok(Instruction::FnLess(attr, value, String::from(out)));
            } else {
                return Ok(Instruction::FnGreater(attr, value, String::from(out)));
            }
        } else if s == "R" {
            return Ok(Instruction::Reject);
        } else if s == "A" {
            return Ok(Instruction::Accept);
        } else {
            return Ok(Instruction::Label(String::from(s)));
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Workflow {
    label: String,
    instructions: Vec<Instruction>,
}

fn pipeline<'a>(
    gear: &'a GearPart,
    pipelines: &HashMap<String, Rc<Workflow>>,
) -> Option<&'a GearPart> {
    let mut opt = pipelines.get("in");
    let mut res = None;

    while let Some(workflow) = opt {
        opt = match workflow.instructions.iter().find_map(|rule| match rule {
            Instruction::FnGreater(c, value, redirect) => match c {
                'x' if *value > gear.x => Some((Some(redirect), None)),
                'm' if *value > gear.m => Some((Some(redirect), None)),
                'a' if *value > gear.a => Some((Some(redirect), None)),
                's' if *value > gear.s => Some((Some(redirect), None)),
                _ => None,
            },
            Instruction::FnLess(c, value, redirect) => match c {
                'x' if *value < gear.x => Some((Some(redirect), None)),
                'm' if *value < gear.m => Some((Some(redirect), None)),
                'a' if *value < gear.a => Some((Some(redirect), None)),
                's' if *value < gear.s => Some((Some(redirect), None)),
                _ => None,
            },
            Instruction::Label(label) => Some((Some(label), None)),

            Instruction::Accept => Some((None, Some(Instruction::Accept))),
            Instruction::Reject => Some((None, Some(Instruction::Reject))),
        }) {
            None => None,
            Some((None, Some(Instruction::Reject))) => None,
            Some((None, Some(Instruction::Accept))) => {
                res = Some(gear);
                None
            }
            Some((Some(label), _)) => pipelines.get(label),
            _ => panic!(),
        };
    }

    return res;
}

fn main() -> std::io::Result<()> {
    /* part 1 */
    let store = Regex::new(r"(.+)\{(.+)\}$").unwrap();
    let r_parts = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();

    let (pipelines, gears) = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| line != "")
        .filter(|line| !line.starts_with("#"))
        .group_by(|line| store.is_match(line))
        .into_iter()
        .fold(
            (HashMap::new(), vec![]),
            |(pipelines, gears), (key, group)| {
                let lines: Vec<String> = group.collect();
                // process pipeline
                if key {
                    let pipelines = lines
                        .iter()
                        .map(|line| {
                            let capture = store.captures_iter(&line).next().unwrap();
                            let (label, rules_str) = capture
                                .extract::<2>()
                                .1
                                .into_iter()
                                .collect_tuple()
                                .unwrap();

                            let rules = rules_str
                                .split(",")
                                .filter_map(|str| Instruction::from_str(str).ok())
                                .collect_vec();

                            (
                                String::from(label),
                                Rc::new(Workflow {
                                    label: String::from(label),
                                    instructions: rules,
                                }),
                            )
                        })
                        .chain(std::iter::once((
                            String::from("A"),
                            Rc::new(Workflow {
                                label: String::from("A"),
                                instructions: vec![Instruction::Accept],
                            }),
                        )))
                        .chain(std::iter::once((
                            String::from("R"),
                            Rc::new(Workflow {
                                label: String::from("R"),
                                instructions: vec![Instruction::Reject],
                            }),
                        )))
                        .collect::<HashMap<String, Rc<Workflow>>>();
                    return (pipelines, gears);
                } else {
                    let gears = lines
                        .iter()
                        .map(|line| {
                            let capture = r_parts.captures_iter(&line).next().unwrap();
                            let (x, m, a, s) = capture
                                .extract::<4>()
                                .1
                                .into_iter()
                                .filter_map(|n| n.parse::<u64>().ok())
                                .collect_tuple()
                                .unwrap();

                            GearPart { x, m, a, s }
                        })
                        .collect_vec();
                    return (pipelines, gears);
                }
            },
        );

    let accepted = gears.iter().filter_map(|gear| pipeline(gear, &pipelines));

    println!("p1: {}", accepted.map(|g| g.score()).sum::<u64>());
    let four_thousands = 4_000;

    let mut possibilities = vec![(
        "in",
        [
            (1..(four_thousands + 1)),
            (1..(four_thousands + 1)),
            (1..(four_thousands + 1)),
            (1..(four_thousands + 1)),
        ],
    )];

    let mut accepted = vec![];

    while let Some((label, mut ranges)) = possibilities.pop() {
        println!("now at: {label:?} with {:?}", (label, &ranges));
        // println!("possibilities: {:?}", possibilities);

        let start = pipelines.get(&label as &str).unwrap();
        let mut out_redirected: Vec<(&str, [_; 4])> = vec![];

        println!("instruction: {:?}", start);
        for rule in start.instructions.iter() {
            match rule {
                Instruction::FnLess(c, value, redirect) => {
                    let idx = match c {
                        'x' => 0,
                        'm' => 1,
                        'a' => 2,
                        's' => 3,
                        _ => panic!(),
                    };

                    if ranges[idx].contains(&(value - 1)) {
                        println!("et ouai {:?} {}", ranges[idx], value);
                        let mut splitted_ranges = ranges.clone();
                        let (out, _in) = (
                            (splitted_ranges[idx].start)..(*value + 1),
                            (*value + 1)..splitted_ranges[idx].end,
                        );

                        splitted_ranges[idx] = _in;
                        out_redirected.push((redirect, splitted_ranges));

                        ranges[idx] = out;
                    }
                }
                Instruction::FnGreater(c, value, redirect) => {
                    let idx = match c {
                        'x' => 0,
                        'm' => 1,
                        'a' => 2,
                        's' => 3,
                        _ => panic!(),
                    };

                    if ranges[idx].contains(&(value - 1)) {
                        println!("et ouai {:?} {}", ranges[idx], value);
                        let mut splitted_ranges = ranges.clone();
                        let (_in, out) = (
                            (splitted_ranges[idx].start)..*value,
                            *value..splitted_ranges[idx].end,
                        );

                        splitted_ranges[idx] = _in;
                        out_redirected.push((redirect, splitted_ranges));

                        ranges[idx] = out;
                    }
                    // possibilities.push((label, splitted_ranges));
                }

                Instruction::Accept => {
                    accepted.push(ranges.clone());
                }
                Instruction::Reject => {}
                Instruction::Label(label) => {
                    out_redirected.push((label, ranges.clone()));
                }
            }
        }

        println!("out of {label:?} {out_redirected:?}");
        possibilities.append(&mut out_redirected);
    }

    println!("{accepted:?}");
    println!(
        "p2: {:?}",
        accepted
            .par_iter()
            .map(|atoms| {
                    (atoms.clone(), atoms.iter().map(|r| r.end - r.start).product::<u64>()
                    )
                    // atoms.clone(),
                    // itertools::iproduct!(
                    //     atoms[0].clone(),
                    //     atoms[1].clone(),
                    //     atoms[2].clone(),
                    //     atoms[3].clone()
                    // )
                    // .count(),
            })
            .inspect(|c| println!("got: {c:?}"))
            .map(|(_, c)| c)
            .sum::<u64>(),
    );
    Ok(())
}
