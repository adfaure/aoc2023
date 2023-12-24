use std::io::BufRead;
use itertools::Itertools;
use regex::Regex;
use std::{fs::File, io::BufReader};


fn main() -> std::io::Result<()> {
    let r_hail = Regex::new(r"(\d+), (\d+), (\d+) @ (\d+), (\d+), (\d+)").unwrap();
    /* part 1 */
    let _grid = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            r_hail.captures_iter(line).next().unwrap().extract::<6>().1.into_iter().collect_tuples().unwrap();
        });

    Ok(())
}
