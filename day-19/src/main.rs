use std::io::BufRead;
use std::{fs::File, io::BufReader};

fn main() -> std::io::Result<()> {

    let input = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|line| line.ok());

    Ok(())
}
