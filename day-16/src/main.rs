use std::io::BufRead;
use std::{fs::File, io::BufReader};

fn main() -> std::io::Result<()> {
   /* part 1 */
    let _ = BufReader::new(File::open("input")?)
        .lines()
        .map(|l| l.unwrap().chars().map(|c| c.to_string()).collect())
        .collect::<Vec<Vec<_>>>();

    Ok(())
}
