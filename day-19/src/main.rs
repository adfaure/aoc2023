use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;
use std::rc::Rc;
use std::str::FromStr;
use std::{fs::File, io::BufReader};

#[derive(Debug, PartialEq, Eq)]
struct GearPart {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl GearPart {
    fn score(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    FnLess(char, u32, String),
    FnGreater(char, u32, String),
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
            let value = number.parse::<u32>().unwrap();
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

fn main() -> std::io::Result<()> {
    /* part 1 */
    let store = Regex::new(r"(.+)\{(.+)\}$").unwrap();
    let r_parts = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();

    let (pipelines, gears) = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| line != "")
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
                                .filter_map(|n| n.parse::<u32>().ok())
                                .collect_tuple()
                                .unwrap();

                            GearPart { x, m, a, s }
                        })
                        .collect_vec();
                    return (pipelines, gears);
                }
            },
        );

    let mut accepted = vec![];
    gears.iter().for_each(|gear| {
        let mut opt = pipelines.get("in");

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
                    accepted.push(gear);
                    None
                }
                Some((Some(label), _)) => pipelines.get(label),
                _ => panic!(),
            };
        }

    });

    println!("p1: {}", accepted.iter().map(|g| g.score()).sum::<u32>() );

    Ok(())
}
