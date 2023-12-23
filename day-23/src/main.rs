use itertools::Itertools;
use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::io::BufRead;
use std::rc::Rc;
use std::{fs::File, io::BufReader};

#[derive(PartialEq, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn to_vec2(&self) -> (i32, i32) {
        match self {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
        }
    }

    fn from_vec2(dir: (i32, i32)) -> Self {
        match dir {
            (0, 1) => Direction::Down,
            (0, -1) => Direction::Up,
            (1, 0) => Direction::Right,
            (-1, 0) => Direction::Left,
            _ => panic!(),
        }
    }
}

fn neighbours<'a>(
    grid: &'a Vec<Vec<char>>,
    pos: &'a (u32, u32),
) -> impl Iterator<Item = (u32, u32)> + 'a {
    return [(0, 1), (0, -1), (-1, 0), (1, 0)]
        .into_iter()
        .map(move |dir| {
            (
                pos.0.checked_add_signed(dir.0),
                pos.1.checked_add_signed(dir.1),
                Direction::from_vec2(dir),
            )
        })
        .filter_map(move |neig| match (neig.0, neig.1) {
            (None, _) | (_, None) => None,
            (Some(x), Some(y)) => {
                if x < grid[0].len() as u32 && y < grid.len() as u32 {
                    let tile = grid[y as usize][x as usize];
                    if tile != '#' {
                        match tile {
                            '.' => Some((x, y)),
                            '>' if neig.2 == Direction::Right => Some((x, y)),
                            '^' if neig.2 == Direction::Up => Some((x, y)),
                            'v' if neig.2 == Direction::Down => Some((x, y)),
                            '<' if neig.2 == Direction::Left => Some((x, y)),
                            _ => None,
                        }
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
        });
}

fn longuest_from(
    grid: &Vec<Vec<char>>,
    pos: &(u32, u32),
    path: HashSet<(u32, u32)>,
    dist: u32,
) -> u32 {
    let mut path = path.clone();
    if path.insert(*pos) {
        neighbours(&grid, pos)
            .map(|new_pos| longuest_from(grid, &new_pos, path.clone(), dist + 1))
            .max()
            .unwrap()
    } else {
        dist
    }
}

fn neighbours_no_slope<'a>(
    grid: &'a Vec<Vec<char>>,
    pos: &'a (u32, u32),
) -> impl Iterator<Item = (u32, u32)> + 'a {
    return [(0, 1), (0, -1), (-1, 0), (1, 0)]
        .into_iter()
        .map(move |dir| {
            (
                pos.0.checked_add_signed(dir.0),
                pos.1.checked_add_signed(dir.1),
            )
        })
        .filter_map(move |neig| match (neig.0, neig.1) {
            (None, _) | (_, None) => None,
            (Some(x), Some(y)) => {
                if x < grid[0].len() as u32
                    && y < grid.len() as u32
                    && grid[y as usize][x as usize] != '#'
                {
                    Some((x, y))
                } else {
                    None
                }
            }
        });
}

fn longuest_from_p2(
    grid: &Vec<Vec<char>>,
    pos: &(u32, u32),
    path: HashSet<(u32, u32)>,
    max_til_now: Rc<Cell<u32>>,
    intersections: &Vec<Vec<bool>>,
) -> u32 {
    let mut path = path.clone();
    let end = (grid[0].len() as u32 - 2, grid.len() as u32 - 1);

    if path.insert(*pos) && pos != &end {
        neighbours_no_slope(&grid, &pos)
            .map(|new_pos| {
                longuest_from_p2(
                    grid,
                    &new_pos,
                    path.clone(),
                    max_til_now.clone(),
                    intersections,
                )
            })
            .max()
            .unwrap()
    } else if pos == &end {
        if max_til_now.get() < path.len() as u32 {
            max_til_now.set(path.len() as u32);
            println!("p2*: {:?}", path.len());
            path.len() as u32
        } else {
            0
        }
    } else {
        0
    }
}

#[derive(Clone)]
struct Node {
    pos: (u32, u32),
    neighbours: RefCell<Vec<(Rc<Node>, u32)>>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}
impl Eq for Node {}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f = f.debug_struct("Node");
        f.field("x", &self.pos);

        for n in self.neighbours.clone().into_inner().iter() {
            f.field("n:", &n.0.pos);
            f.field("dist:", &n.1);
        }

        f.finish()
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
    }
}

impl Node {
    fn print_all(&self, mut printed: &mut HashSet<Rc<Node>>) {
        let mut fifo = VecDeque::from_iter(vec![Rc::new(self.clone())].into_iter());

        while let Some(node) = fifo.pop_front() {
            if printed.insert(node.clone()) {
                print!("print: {:?} [", node.pos);
                for n in node.neighbours.clone().into_inner().iter() {
                    print!(" {:?} -> {}", n.0.pos, n.1);
                    fifo.push_back(n.0.clone());
                }
                println!(" ]")
            }
        }
    }
}

fn solve_p2(grid: &Vec<Vec<char>>) -> Vec<Vec<bool>> {
    let mut res = vec![vec![false; grid[0].len()]; grid.len()];

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let nb_neighbors = neighbours_no_slope(&grid, &(x as u32, y as u32)).count();
            if nb_neighbors > 2 && grid[y][x] != '#' {
                // print!("0");
                res[y][x] = true;
            } else {
                // print!("{}", grid[y][x]);
            }
        }
        // println!();
    }

    let mut seen = HashSet::new();

    let current = (1, 0);
    let start_node = Rc::new(Node {
        pos: current,
        neighbours: RefCell::new(vec![]),
    });

    let mut nodes: HashMap<(u32, u32), Rc<Node>> = HashMap::new();
    nodes.insert(current, start_node.clone());

    let mut stack = vec![(start_node.clone(), current, 0)];

    while let Some((node, current, dist)) = stack.pop() {
        if seen.insert((node.pos, current)) {
            if !res[current.1 as usize][current.0 as usize] {
                let next = neighbours_no_slope(grid, &current)
                    .filter(|neig| !seen.contains(&(node.pos, *neig)))
                    .next();

                match next {
                    None => {
                        // println!("None at from {node:?} {current:?}");
                        if node.pos != current {
                            let new_node: Rc<Node> = match nodes.get(&current) {
                                None => {
                                    let new = Rc::new(Node {
                                        pos: current,
                                        neighbours: RefCell::new(vec![(node.clone(), dist)]),
                                    });
                                    nodes.insert(current, new.clone());
                                    new
                                }
                                Some(n) => n.clone(),
                            };

                            node.neighbours.borrow_mut().push((new_node.clone(), dist));
                        } else {
                            panic!()
                        }
                    }
                    Some(pos) => {
                        // println!("Continue with node {node:?} at {current:?}");
                        stack.push((node, pos, dist + 1));
                    }
                }
            } else {
                // println!("Currently at intersection: {current:?}");
                assert!(neighbours_no_slope(grid, &current).count() > 1);

                let new_node: Rc<Node> = match nodes.get(&current) {
                    None => {
                        // println!("create node at {current:?}");
                        let new = Rc::new(Node {
                            pos: current,
                            neighbours: RefCell::new(vec![(node.clone(), dist)]),
                        });
                        nodes.insert(current, new.clone());
                        new
                    }
                    Some(n) => n.clone(),
                };

                node.neighbours.borrow_mut().push((new_node.clone(), dist));
                seen.insert((current, current));

                stack.extend(
                    neighbours_no_slope(grid, &current)
                        .map(|neig_pos| (new_node.clone(), neig_pos, 0)),
                );
            }
        }
    }

    // println!("Graph:");
    // start_node.print_all(&mut HashSet::new());
    // println!("all printed");

    let end = (grid[0].len() as u32 - 2, grid.len() as u32 - 1);
    println!(
        "p2: {end:?} {:?}",
        p2_graph(
            start_node,
            &end,
            vec![],
            HashSet::new(),
            0,
            Cell::new(0).into(),
            &mut HashMap::new()
        ) - 1
    );

    return res;
}

fn p2_graph(
    node: Rc<Node>,
    end: &(u32, u32),
    path: Vec<(u32, u32)>,
    seen: HashSet<(u32, u32)>,
    dist: u32,
    max_til_now: Rc<Cell<u32>>,
    memo: &mut HashMap<Vec<(u32, u32)>, u32>,
) -> u32 {
    let mut seen = seen.clone();
    let mut path = path.clone();

    path.push(node.pos);
    if memo.contains_key(&path) {
        return *memo.get(&path).unwrap();
    }

    if seen.insert(node.pos) {
        if node.pos != *end {
            let max = node
                .neighbours
                .clone()
                .into_inner()
                .iter()
                .map(|(new_node, to_dist)| {
                    p2_graph(
                        new_node.clone(),
                        end,
                        path.clone(),
                        seen.clone(),
                        dist + to_dist + 1,
                        max_til_now.clone(),
                        memo,
                    )
                })
                .max()
                .unwrap();

            memo.insert(path, max);
            max
        } else {
            if max_til_now.get() < dist as u32 {
                memo.insert(path, dist);
                max_til_now.set(dist as u32);
                println!("p2*: {:?}", dist - 1);
                dist as u32
            } else {
                memo.insert(path, 0);
                0
            }
        }
    } else {
        0
    }
}

fn solve_p1(grid: Vec<Vec<char>>, pos: &(u32, u32)) {
    let res = longuest_from(&grid, pos, HashSet::new(), 0);

    println!("p1: {}", res - 1);
}

fn main() -> std::io::Result<()> {
    /* part 1 */

    let _grid = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    solve_p1(_grid.clone(), &(1, 0));
    solve_p2(&_grid);

    Ok(())
}
