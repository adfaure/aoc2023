use regex::Regex;
use std::io::BufRead;

use std::{fs::File, io::BufReader};


fn main() -> std::io::Result<()> {
    /* part 1 */
    let res = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|l| {
            let first = l
                .as_ref()
                .unwrap()
                .chars()
                .find_map(|c| (c.to_string()).parse::<i32>().ok());

            let last = l
                .as_ref()
                .unwrap()
                .chars()
                .rev()
                .find_map(|c| (c.to_string()).parse::<i32>().ok());

            match (last, first) {
                (Some(last), Some(first)) => Some(last + first * 10),
                (None, _) | (_, None) => None,
            }
        })
        .sum::<i32>();
    println!("p1: {:?}", res);

    let r_first = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|zero|\d)").unwrap();
    let r_last =
        Regex::new(r".*(one|two|three|four|five|six|seven|eight|nine|zero|\d)").unwrap();

    let res = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| {
            let last: i32 = match r_last.captures(&l) {
                None => panic!(""),
                Some(capture) => match capture.get(1).unwrap().as_str() {
                    "zero" => 0,
                    "one" => 1,
                    "two" => 2,
                    "three" => 3,
                    "four" => 4,
                    "five" => 5,
                    "six" => 6,
                    "seven" => 7,
                    "eight" => 8,
                    "nine" => 9,
                    other => other.parse::<i32>().unwrap(),
                },
            };

            let first = match r_first.captures(&l) {
                None => panic!(""),
                Some(capture) => match capture.get(1).unwrap().as_str() {
                    "zero" => 0,
                    "one" => 1,
                    "two" => 2,
                    "three" => 3,
                    "four" => 4,
                    "five" => 5,
                    "six" => 6,
                    "seven" => 7,
                    "eight" => 8,
                    "nine" => 9,
                    other => other.parse::<i32>().unwrap(),
                },
            };

            10 * first + last
        })
        .sum::<i32>();

    println!("p2: {:?}", res);
    Ok(())
}
