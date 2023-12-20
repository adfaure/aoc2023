use itertools::Itertools;
use regex::Regex;
use std::collections::VecDeque;
use std::cell::Cell;
use std::collections::HashMap;
use std::io::BufRead;
use std::rc::Rc;
use std::str::FromStr;
use std::{fs::File, io::BufReader};

#[derive(Debug, Clone, Copy)]
enum Signal {
    Low,
    High,
}

#[derive(Debug)]
enum Module {
    Inv(String, Vec<String>, Vec<Signal>),
    FlipFlop(String, Vec<String>, bool),
    BroadCast(Vec<String>),
}

impl Module {
    fn label(&self) -> &str {
        match self {
            Module::Inv(label, _, _) => label,
            Module::FlipFlop(label, _, _) => label,
            Module::BroadCast(_) => "broadcast",
        }
    }

    fn process(&self, in_signals: &Vec<Signal>) -> Vec<(String, Signal)> {
            match self {
                Module::Inv { }
            }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseModuleError;

impl FromStr for Module {
    type Err = ParseModuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r_parse = Regex::new(r"([%&]?)(.*) -> (.+)$").unwrap();

        if r_parse.is_match(s) {
            let (op, label, out) = r_parse
                .captures_iter(&s)
                .next()
                .unwrap()
                .extract::<3>()
                .1
                .into_iter()
                .collect_tuple()
                .unwrap();

            println!("op: {:?}, {:?}", label, op);
            let out_labels = out.split(",").map(|label| label.to_string()).collect_vec();
            let module = match op {
                "%" => Module::FlipFlop(label.to_string(), out_labels, false),
                "&" => Module::Inv(label.to_string(), out_labels, vec![]),
                _ => Module::BroadCast(out.split(",").map(|label| label.to_string()).collect_vec()),
            };

            return Ok(module);
        }
        Err(ParseModuleError)
    }
}

fn main() -> std::io::Result<()> {
    /* part 1 */
    let store = Regex::new(r"(.+) -> (.+)$").unwrap();

    let modules = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| line != "")
        .filter(|line| !line.starts_with("#"))
        .into_iter()
        .filter_map(|line| Module::from_str(&line).ok())
        .map(|module| (module.label().to_string(), module))
        .collect::<HashMap<String, Module>>();

    println!("{:?}", modules);

    let start = modules.get("broadcast").unwrap();

    // Vec of signals entering a module
    let mut signals: VecDeque<Vec<((String, Signal))>> = VecDeque::new();

    while let Some(mut signals) = signals.pop_front() {
        signals.sort_by(|l, r| l.0.cmp(&r.0));
        signals.iter().group_by(|(label, _)| label).into_iter().for_each(|(key, group)| {
            let label: Vec<&(String, Signal)> = group.collect();

        })
    }


    Ok(())
}
