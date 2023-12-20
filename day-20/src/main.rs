use itertools::Itertools;
use regex::Regex;
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
    Inv(String, String, Vec<Signal>),
    FlipFlop(String, String, bool),
    BroadCast(Vec<String>),
    Button(String),
}

#[derive(Debug, PartialEq, Eq)]
struct ParseModuleError;

impl FromStr for Module {
    type Err = ParseModuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fn_parse = Regex::new(r"([%&]?)(.*)$").unwrap();
        if fn_parse.is_match(s) {
            let (op, label) = fn_parse
                .captures_iter(&s)
                .next()
                .unwrap()
                .extract::<2>()
                .1
                .into_iter()
                .collect_tuple()
                .unwrap();

            println!("op: {:?}, {:?}", label, op);
        }
        Ok(Module::Button("".to_string()))
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
        .map(|line| {
            let capture = store.captures_iter(&line).next().unwrap();
            let (module_str, outputs) = capture
                .extract::<2>()
                .1
                .into_iter()
                .collect_tuple()
                .unwrap();
            let module = Module::from_str(module_str);
            ()
        })
        .collect_vec();

    println!("{:?}", modules);

    Ok(())
}
