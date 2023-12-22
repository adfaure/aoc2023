use itertools::Itertools;
use std::io::BufRead;
use std::{fs::File, io::BufReader};

fn main() -> std::io::Result<()> {
    /* part 1 */

    let input = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|line| line.ok())
        .collect_vec();

    Ok(())
}
