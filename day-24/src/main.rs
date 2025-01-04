use itertools::Itertools;
use regex::Regex;
use std::io::BufRead;
use std::{fs::File, io::BufReader};
use z3::ast::{Ast, Int};
use z3::Params;
use z3::{Config, Context};

#[derive(Debug, Clone, Copy)]
struct Hails {
    pos: (f64, f64, f64),
    vel: (f64, f64, f64),
}

fn collide_2d(line1: Hails, point: (f64, f64, f64)) -> bool {
    let dot = ((point.0 - line1.pos.0) * line1.vel.0) + ((point.1 - line1.pos.1) * line1.vel.1);
    // println!("line: {line1:?} point: {point:?} dot: {dot:?}");
    !(dot < 1e-10)
}

fn collide_lines_2d(line1: Hails, line2: Hails) -> Option<(f64, f64)> {
    // println!("{line1:?} {line2:?}");
    let a = line1.vel.0;
    let b = -line2.vel.0;
    let c = line1.vel.1;
    let d = -line2.vel.1;

    let det = a * d - b * c;

    if det.abs() < 1e-10 {
        // Lines are parallel or coincident
        return None;
    }

    let e = line2.pos.0 - line1.pos.0;
    let f = line2.pos.1 - line1.pos.1;

    let t = (e * d - b * f) / det;

    // Validate by plugging t and u back into the third equation
    // if true || (line1.pos.2 + t * line1.vel.2 - (line2.pos.2 + u * line2.vel.2)).abs() < 1e-10 {
    //
    let intersection = (
        line1.pos.0 + t * line1.vel.0,
        line1.pos.1 + t * line1.vel.1,
        0.0, // line1.pos.2 + t * line1.vel.2,
    );

    if collide_2d(line1, intersection) && collide_2d(line2, intersection) {
        Some((intersection.0, intersection.1))
    } else {
        None
    }
}

fn main() -> std::io::Result<()> {
    let r_hail =
        Regex::new(r"(\d+),[ ]+(\d+),[ ]+(\d+)[ ]+@[ ]+([-]?\d+),[ ]+([-]?\d+),[ ]+([-]?\d+)")
            .unwrap();

    let boundaires = (200000000000000.0, 400000000000000.0);
    /* part 1 */
    let hails = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            // println!("{:?}", line);
            let (x, y, z, vx, vy, vz) = r_hail
                .captures_iter(&line)
                .next()
                .unwrap()
                .extract::<6>()
                .1
                .into_iter()
                .filter_map(|n| n.parse::<f64>().ok())
                .collect_tuple()
                .unwrap();

            Hails {
                pos: (x, y, z),
                vel: (vx, vy, vz),
            }
        })
        .combinations(2)
        .filter_map(|combination| collide_lines_2d(combination[0], combination[1]))
        .filter(|intersection| {
            intersection.0 > boundaires.0
                && intersection.0 < boundaires.1
                && intersection.1 > boundaires.0
                && intersection.1 < boundaires.1
        })
        // .inspect(|e| println!("{e:?}"))
        .count();

    // println!("{:?}", collide_point_2d(hails[0].clone(), hails[1].clone()));
    println!("p1: {:?}", hails);

    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let solver = z3::Solver::new(&ctx);
    let mut params = Params::new(&ctx);
    params.set_bool("parallel.enable", true); // Enable parallel solving
    params.set_u32("parallel.threads.max", 4); // Use up to 4 threads
    params.set_bool("parallel.enable", true);
    // params.set_bool("model", true);
    // params.set_bool("maximize_memory_usage", true); // Allow more memory usag
    solver.set_params(&params);
    let x = z3::ast::Int::new_const(&ctx, format!("x"));
    let y = z3::ast::Int::new_const(&ctx, format!("y"));
    let z = z3::ast::Int::new_const(&ctx, format!("z"));

    let vx = z3::ast::Int::new_const(&ctx, format!("vx"));
    let vy = z3::ast::Int::new_const(&ctx, format!("vy"));
    let vz = z3::ast::Int::new_const(&ctx, format!("vz"));

    let boundaries = (
        z3::ast::Int::from_u64(&ctx, boundaires.0 as u64),
        z3::ast::Int::from_u64(&ctx, boundaires.1 as u64),
    );

    solver.assert(&x.ge(&boundaries.0));
    solver.assert(&x.le(&boundaries.1));

    solver.assert(&y.ge(&boundaries.0));
    solver.assert(&y.le(&boundaries.1));

    // solver.assert(&z.ge(&boundaries.0));
    // solver.assert(&z.le(&boundaries.1));

    let time_vars: Vec<_> = (0..300)
        .map(|i| Int::new_const(&ctx, format!("t{}", i)))
        .collect();

    let hails = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            // println!("{:?}", line);
            let (x, y, z, vx, vy, vz) = r_hail
                .captures_iter(&line)
                .next()
                .unwrap()
                .extract::<6>()
                .1
                .into_iter()
                .filter_map(|n| n.parse::<f64>().ok())
                .collect_tuple()
                .unwrap();

            Hails {
                pos: (x, y, z),
                vel: (vx, vy, vz),
            }
        })
        .take(3)
        .enumerate()
        .for_each(|(idx, hail)| {
            // println!("hail: {hail:?}");
            let t1 = &time_vars[idx];

            let hail_x = z3::ast::Int::from_i64(&ctx, hail.pos.0 as i64);
            let hail_vx = z3::ast::Int::from_i64(&ctx, hail.vel.0 as i64);
            let hail_y = z3::ast::Int::from_i64(&ctx, hail.pos.1 as i64);
            let hail_vy = z3::ast::Int::from_i64(&ctx, hail.vel.1 as i64);
            let hail_z = z3::ast::Int::from_i64(&ctx, hail.pos.2 as i64);
            let hail_vz = z3::ast::Int::from_i64(&ctx, hail.vel.2 as i64);

            // More efficient constraint encoding
            let dx = &x - &hail_x;
            let dy = &y - &hail_y;
            let dz = &z - &hail_z;

            let dvx = &vx - &hail_vx;
            let dvy = &vy - &hail_vy;
            let dvz = &vz - &hail_vz;

            solver.assert(&dx._eq(&(t1 * &dvx)));
            solver.assert(&dy._eq(&(t1 * &dvy)));
            solver.assert(&dz._eq(&(t1 * &dvz)));

        });

    // Check if satisfiable first
    match solver.check() {
        z3::SatResult::Sat => {
            let model = solver.get_model().unwrap();
            let result = (
                model.eval(&x, true).unwrap(),
                model.eval(&y, true).unwrap(),
                model.eval(&z, true).unwrap(),
            );

            println!(
                "p2 ({:?}): {}",
                &result,
                &result.0.as_i64().unwrap()
                    + &result.1.as_i64().unwrap()
                    + &result.2.as_i64().unwrap()
            );
        }
        z3::SatResult::Unsat => println!("No solution exists!"),
        z3::SatResult::Unknown => println!("Failed to find a solution"),
    }

    Ok(())
}
