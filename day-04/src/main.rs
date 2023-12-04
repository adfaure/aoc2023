use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;
use std::io::BufRead;

use std::{fs::File, io::BufReader};

fn main() -> std::io::Result<()> {
    let res: i32 = BufReader::new(File::open("input")?)
        .lines()
        .map(|l| {
            let content = l
                .unwrap()
                .chars()
                .skip_while(|c| *c != ':')
                .collect::<String>();
            content
                .split("|")
                .fold((HashSet::new(), HashSet::new()), |mut acc, numbers| {
                    if acc.0.is_empty() {
                        acc.0 = numbers
                            .split(" ")
                            .filter_map(|value| value.parse::<i32>().ok())
                            .collect();
                    } else {
                        acc.1 = numbers
                            .split(" ")
                            .filter_map(|value| value.parse::<i32>().ok())
                            .collect();
                    }
                    acc
                })
        })
        .filter(|(winning, mine)| !winning.is_disjoint(&mine))
        .map(|(winning, mine)| i32::pow(2, winning.intersection(&mine).count() as u32 - 1))
        .sum();
    println!("p1: {:?}", res);

    let res: i32 = BufReader::new(File::open("input")?)
        .lines()
        .enumerate()
        .map(|(i, l)| {
            let content = l
                .unwrap()
                .chars()
                .skip_while(|c| *c != ':')
                .collect::<String>();
            (
                i + 1,
                content
                    .split("|")
                    .fold((HashSet::new(), HashSet::new()), |mut acc, numbers| {
                        if acc.0.is_empty() {
                            acc.0 = numbers
                                .split(" ")
                                .filter_map(|value| value.parse::<i32>().ok())
                                .collect();
                        } else {
                            acc.1 = numbers
                                .split(" ")
                                .filter_map(|value| value.parse::<i32>().ok())
                                .collect();
                        }
                        acc
                    }),
            )
        })
        .fold(
            HashMap::new() as HashMap<i32, i32>,
            |mut cards, (card_id, (winning, mine))| {
                {
                    let total_card_id_update = cards.entry(card_id as i32).or_insert(0);
                    *total_card_id_update += 1;
                }
                let total_card_id: i32 = *cards.get(&(card_id as i32)).unwrap();

                let count = winning.intersection(&mine).count();
                // println!("parsing card: {card_id} - {winning:?} / {mine:?} = {count}");
                for i in (card_id + 1)..=(card_id + count) {
                    let total = cards.entry(i as i32).or_insert(0);
                    *total += total_card_id;
                    // println!("card: {i}: {total}");
                }
                // println!("{card_id}: {cards:?}");
                cards
            },
        )
        .iter()
        //.inspect(|(k, v)| println!("I have {v} of {k}"))
        .map(|(_, v)| v)
        .sum();

    println!("p2: {:?}", res);
    Ok(())
}
