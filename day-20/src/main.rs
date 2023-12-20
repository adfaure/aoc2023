use itertools::Itertools;
use rayon::iter::plumbing::Consumer;
use regex::Regex;
use std::borrow::BorrowMut;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::BufRead;
use std::rc::Rc;
use std::str::FromStr;
use std::{fs::File, io::BufReader};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Signal {
    Low,
    High,
}

#[derive(Debug, Clone)]
enum Module {
    Inv(String, Vec<String>, RefCell<HashMap<String, Signal>>),
    FlipFlop(String, Vec<String>, Cell<bool>),
    BroadCast(Vec<String>),
}

impl Module {
    fn label(&self) -> &str {
        match self {
            Module::Inv(label, _, _) => label,
            Module::FlipFlop(label, _, _) => label,
            Module::BroadCast(_) => "broadcaster",
        }
    }

    fn out(&self) -> &[String] {
        match self {
            Module::Inv(_, out, _) => out,
            Module::FlipFlop(_, out, _) => out,
            Module::BroadCast(out) => out,
        }
    }

    fn process(&self, in_signal: &(String, Signal)) -> Vec<(String, String, Signal)> {
        match self {
            Module::BroadCast(out) => out
                .iter()
                .map(|out_label| (self.label().to_string(), out_label.clone(), in_signal.1))
                .collect_vec(),
            Module::FlipFlop(_, out, on_off) => {
                if in_signal.1 == Signal::Low {
                    let to_send;
                    if on_off.get() == true {
                        on_off.set(false);
                        to_send = Signal::Low;
                    } else {
                        on_off.set(true);
                        to_send = Signal::High;
                    }
                    out.iter()
                        .map(|label| (self.label().to_string(), label.clone(), to_send))
                        .collect_vec()
                } else {
                    vec![]
                }
            }
            Module::Inv(_, out, rcell_memo) => {
                let mut memo = rcell_memo.borrow_mut();

                memo.insert(in_signal.0.clone(), in_signal.1);

                let sig_to_send = if memo.iter().all(|(_, s)| s == &Signal::High) {
                    Signal::Low
                } else {
                    Signal::High
                };

                out.iter()
                    .map(|label| (self.label().to_string(), label.clone(), sig_to_send))
                    .collect_vec()
            }
            _ => vec![],
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
            let out_labels = out
                .split(",")
                .map(|l| l.trim())
                .map(|label| label.to_string())
                .collect_vec();
            let module = match op {
                "%" => Module::FlipFlop(label.to_string(), out_labels, Cell::new(false)),
                "&" => Module::Inv(label.to_string(), out_labels, RefCell::new(HashMap::new())),
                _ => Module::BroadCast(out_labels),
            };

            return Ok(module);
        }
        Err(ParseModuleError)
    }
}

fn push_button(modules: &HashMap<String, Rc<Module>>, iter_no: u64) -> Option<(u64, u64)> {
    // Vec of signals entering a module (from, to, sig)
    let mut next_stage: VecDeque<(String, String, Signal)> =
        VecDeque::from([("button".to_string(), "broadcaster".to_string(), Signal::Low)].to_vec());

    let (mut score_high, mut score_low) = (0, 0);
    while let Some((from, to, sig)) = next_stage.pop_front() {
        //  signals.sort_by(|l, r| l.0.cmp(&r.0));
        // println!("signals: {:?}", next_stage);
        // println!("{:?}", modules);

        if sig == Signal::Low {
            score_low += 1;
        } else {
            score_high += 1;
        }

        match modules.get(&to) {
            Some(module) => {
                let emits = module.process(&(from, sig));

                match module.as_ref() {
                    Module::Inv(from, out, memo) => {
                       let (_, _, sig) = emits.first().unwrap();
                       if to == "pr" {
                            // Means that all memo are high
                            println!("module {:?}: sends {sig:?} at {iter_no}", module.label());
                        }
                    }
                    _ => {}
                };

                next_stage.append(&mut VecDeque::from(emits));
            }
            None => {
                if sig != Signal::High {
                    return None
                }
                // rx reached
            }
        }
    }

    Some((score_low, score_high))
}


// 
fn main() -> std::io::Result<()> {
    /* part 1 */
    let modules = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| line != "")
        .filter(|line| !line.starts_with("#"))
        .into_iter()
        .filter_map(|line| Module::from_str(&line).ok())
        .map(|module| (module.label().to_string(), Rc::new(module)))
        .collect::<HashMap<String, Rc<Module>>>();

    // Initialize the inv modules
    modules
        .iter()
        .filter(|(_, m)| match m.as_ref() {
            Module::Inv(_, _, _) => true,
            _ => false,
        })
        .map(|(k, m)| {
            modules
                .iter()
                .filter(|(_, m)| m.out().contains(k))
                .for_each(|(k_in_out, _)| {
                    let module: &Module = m.as_ref();
                    match module {
                        Module::Inv(_, _, inputs) => {
                            (inputs.borrow_mut()).insert(k_in_out.clone(), Signal::Low);
                        }
                        _ => panic!(),
                    }
                });
        })
        .for_each(|_| {});

    // println!("{:?}", modules);

    let (mut l, mut h) = (0, 0);

    for i in 0.. {
        let (l_tmp, h_tmp) = match push_button(&modules, i) {
            Some((a, b)) => (a, b),
            None => panic!("rx reached after: {i}")
        };

        l += l_tmp;
        h += h_tmp;
    }

    println!("p1: {} l:{l} h:{h}", l * h);
    Ok(())
}
