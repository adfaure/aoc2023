use itertools::Itertools;
use std::io::BufRead;
use std::{fs::File, io::BufReader};

fn main() -> std::io::Result<()> {
    let mut grid: Vec<Vec<char>> = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let (size_x, size_y) = (grid[0].len(), grid.len());

    let empty_lines = grid
        .iter()
        .enumerate()
        .filter(|(_y, line)| line.iter().all(|c| *c == '.'))
        .map(|(y, _line)| y)
        .collect_vec();

    let empty_rows = (0..size_x)
        .filter(|x| grid.iter().map(|line| line[*x]).all(|c| c == '.'))
        .collect_vec();

    empty_lines.iter().rev().for_each(|x| {
        let clone = grid[*x as usize].clone();
        grid.insert(*x as usize, clone);
    });

    empty_rows.iter().rev().for_each(|y| {
        let empty = '.';
        grid.iter_mut()
            .for_each(|line| line.insert(*y as usize, empty));
    });

    println!("empty lines: {:?}", empty_lines);
    println!("empty rows: {:?}", empty_rows);

    grid.iter()
        .map(|line| println!("{:?}", line.iter().join("")))
        .collect_vec();

    let galaxies = grid
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(x, c)| if *c == '#' { Some((x, y)) } else { None })
                .collect_vec()
        })
        .enumerate()
        .collect_vec();

    let combinations: i32 = galaxies
        .iter()
        .combinations(2)
        // .inspect(|comb| print!("{comb:?}"))
        .map(|comb| comb.into_iter().collect_tuple().unwrap())
        .map(|(g1, g2)| (g1.1, g2.1))
        .map(|(g1, g2)| (g1.0.abs_diff(g2.0) + g1.1.abs_diff(g2.1)) as i32)
    // .inspect(|dist| println!("= {dist}"))
        .sum();

    println!("p1: {}", combinations);
    Ok(())
}
