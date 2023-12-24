use itertools::Itertools;
use regex::Regex;
use std::io::BufRead;
use std::{fs::File, io::BufReader};

fn main() -> std::io::Result<()> {
    let r_hail =
        Regex::new(r"(\d+),[ ]+(\d+),[ ]+(\d+)[ ]+@[ ]+([-]?\d+),[ ]+([-]?\d+),[ ]+([-]?\d+)")
            .unwrap();
    /* part 1 */
    let hails = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            println!("{:?}", line);
            let (x, y, z, vx, vy, vz) = r_hail
                .captures_iter(&line)
                .next()
                .unwrap()
                .extract::<6>()
                .1
                .into_iter()
                .filter_map(|n| n.parse::<i32>().ok())
                .collect_tuple()
                .unwrap();

            ((x, y, z), (vx, vy, vz))
        })
        .collect_vec();

    println!("hails: {:?}", hails);
    Ok(())
}
