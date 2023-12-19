use itertools::Itertools;
use regex::Regex;
use std::io::BufRead;
use std::str::FromStr;
use std::{fs::File, io::BufReader};

#[derive(Debug, PartialEq, Eq)]
struct GearPart {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    FnLess((char, i32, String)),
    FnGreater((char, i32, String)),
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
            let value = number.parse::<i32>().unwrap();
            if op == '>' {
                return Ok(Instruction::FnGreater((attr, value, String::from(out))));
            } else {
                return Ok(Instruction::FnLess((attr, value, String::from(out))));
            }

        } else if s == "R" {
            return Ok(Instruction::Reject);
        } else if s == "A" {
            return Ok(Instruction::Accept);
        } else {
            return Ok(Instruction::Label(String::from(s)));
        }
        Err(ParseInstuctionError)
    }
}

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
        .map(|(key, group)| {
            let lines: Vec<String> = group.collect();
            println!("{key:?} {:?}", lines);
            // process pipeline
            if key {
                let pipelines = lines
                    .iter()
                    .map(|line| {
                        println!("{line:?}");
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
                        println!("{label:?} {rules:?}");
                    })
                    .collect_vec();
                return vec![];
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
                println!("{:?}", gears);
                return gears;
            }
        })
        .collect_tuple()
        .unwrap();

    println!("pipelines: {:?} - parts {:?}", pipelines, gears);
    Ok(())
}
