use std::collections::HashMap;
use std::io::BufRead;
use std::{fs::File, io::BufReader};

fn main() -> std::io::Result<()> {
    let mut buffer = String::with_capacity(2048);
    let _ = BufReader::new(File::open("input")?).read_line(&mut buffer);

    let result: u32 = buffer
        .split(",")
        .map(|step| step.trim())
        // .inspect(|code| print!("{code:?} = "))
        .map(|step| {
            step.chars().fold(0, |mut acc, c| {
                acc += c as u32;
                acc = acc * 17;
                acc = acc % 256;

                acc
            })
        })
        // .inspect(|hash| println!("{hash}"))
        .sum();
    println!("p1: {result}");

    let mut boxes: HashMap<u32, Vec<(String, u32)>> = (0..256).map(|i| (i, Vec::new())).collect();
    let _ = buffer
        .split(",")
        .map(|step| step.trim())
        .map(|step| {
            let label = step
                .chars()
                .take_while(|c| c != &'=' && c != &'-')
                .collect::<String>();

            (
                label.chars().fold(0, |mut acc, c| {
                    acc += c as u32;
                    acc = acc * 17;
                    acc = acc % 256;

                    acc
                }),
                label,
                step.chars()
                    .skip_while(|c| c != &'=' && c != &'-')
                    .collect::<String>(),
            )
        })
        .for_each(|(box_id, label, op)| {
            // println!("{box_id}, {label:?}, {op}");
            let lens_box = boxes.get_mut(&box_id).unwrap();
            let idx = lens_box.iter().enumerate().find_map(|(index, box_label)| {
                if label == box_label.0 {
                    return Some(index);
                } else {
                    return None;
                }
            });

            match op.as_str() {
                "-" => {
                    if idx.is_some() {
                        lens_box.remove(idx.unwrap());
                    }
                }
                eq_ => {
                    let focal = eq_.chars().skip(1).collect::<String>().parse::<u32>().unwrap();
                    if idx.is_some() {
                        lens_box[idx.unwrap()].1 = focal;
                    } else {
                        lens_box.push((label, focal));
                    }
                }
            };
            // println!("after {lens_box:?}");
        });

    let filtered_boxes : u32 = boxes
        .iter()
        .filter(|(_, v)| !v.is_empty())
        .map(|(box_id, content)| content.iter().enumerate().map(|(idx, (_, focal))| (*box_id, 1 + idx as u32,focal)))
        .flatten()
        .map(|(box_id, idx, focal)| (1 + box_id) * idx * focal)
        .sum();

    println!("p2: {filtered_boxes:?}");

    Ok(())
}
