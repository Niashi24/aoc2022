use std::collections::HashSet;
use cgmath::num_traits::real::Real;
use cgmath::Vector2;
use crate::day17::Jet::{Left, Right};
use crate::day17::Rock::{IHor, IVert, L, O, X};
use crate::day::Day;

pub struct Day17;

pub enum Jet {
    Left,
    Right
}

pub enum Rock {
    IHor,
    X,
    L,
    IVert,
    O
}

const IHOR_ARR: [Vector2<u64>; 4] = [
    Vector2 {x: 0, y: 0},
    Vector2 {x: 1, y: 0},
    Vector2 {x: 2, y: 0},
    Vector2 {x: 3, y: 0},
];

const X_ARR: [Vector2<u64>; 4] = [
    Vector2 {x: 0, y: 1},
    Vector2 {x: 1, y: 0},
    Vector2 {x: 2, y: 1},
    Vector2 {x: 1, y: 2},
    //Vector2 {x: 1, y: 1},  // middle square is covered by other squares
];

const L_ARR: [Vector2<u64>; 5] = [
    Vector2 {x: 0, y: 0},
    Vector2 {x: 1, y: 0},
    Vector2 {x: 2, y: 0},
    Vector2 {x: 2, y: 1},
    Vector2 {x: 2, y: 2},
];

const IVERT_ARR: [Vector2<u64>; 4] = [
    Vector2 {x: 0, y: 0},
    Vector2 {x: 0, y: 1},
    Vector2 {x: 0, y: 2},
    Vector2 {x: 0, y: 3},
];

const O_ARR: [Vector2<u64>; 4] = [
    Vector2 {x: 0, y: 0},
    Vector2 {x: 1, y: 0},
    Vector2 {x: 0, y: 1},
    Vector2 {x: 1, y: 1},
];

fn rock_to_squares(rock: &Rock) -> core::slice::Iter<Vector2<u64>> {
    match rock {
        Rock::IHor => IHOR_ARR.iter(),
        Rock::X => X_ARR.iter(),
        Rock::L => L_ARR.iter(),
        Rock::IVert => IVERT_ARR.iter(),
        Rock::O => O_ARR.iter(),
    }
}

pub struct Info {
    jet_pattern: Vec<Jet>
}

impl Day<Info> for Day17 {
    fn parse_file(&self, file_content: String) -> Info {
        Info {
            jet_pattern: file_content.chars().map(|x| match x {
                '>' => Right,
                '<' => Left,
                _ => panic!("{}", x as u8)
            }).collect()
        }
    }

    fn part_1(&self, data: &Info) -> usize {
        
        const ROCK_LIMIT: u64 = 2022;

        let mut blocks_set: HashSet<Vector2<u64>> = HashSet::new();
        let mut y_max = 0;

        let rocks = [IHor, X, L, IVert, O];
        let mut rocks = rocks.iter().cycle();

        let mut pos = Vector2 {x: 2, y: y_max + 4};
        let mut rock = rocks.next().unwrap();
        let mut rock_count = 0;
        for jet in data.jet_pattern.iter().cycle() {

            match jet {
                Left => {
                    if pos.x != 0 {
                        pos.x -= 1;
                        if intersects(pos, rock, &mut blocks_set) {
                            pos.x += 1;
                        }
                    }
                }
                Right => {
                    if pos.x != 6 {
                        pos.x += 1;
                        if intersects(pos, rock, &mut blocks_set) {
                            pos.x -= 1;
                        }
                    }
                }
            }

            pos.y -= 1;
            if intersects(pos, rock, &mut blocks_set) || pos.y == 0 {
                pos.y += 1;
                rock_count += 1;

                y_max = insert_rock(pos, rock, y_max, &mut blocks_set);

                if rock_count == ROCK_LIMIT {
                    break;
                }

                rock = rocks.next().unwrap();
                pos = Vector2 {x: 2, y: y_max + 4};
            }
        }

        y_max as usize
    }

    fn part_2(&self, data: &Info) -> usize {
        const ROCK_LIMIT: u64 = 1000000000000;

        let mut blocks_set: HashSet<Vector2<u64>> = HashSet::new();
        let mut y_max = 0;

        let rocks = [IHor, X, L, IVert, O];
        let mut rocks = rocks.iter().cycle();

        let mut pos = Vector2 {x: 2, y: y_max + 4};
        let mut rock = rocks.next().unwrap();
        let mut rock_count = 0;
        // Cycle stuff (Part 2)
        // how many cycles it takes to become stable and start a loop
        // number is arbitrary, just enough to buffer past initial chaos
        const CYCLES_BEFORE_STABLE: u32 = 10;

        let mut cycle_num = 0;
        let mut cycle = Vec::new();
        let mut y_increase = 0;
        let mut did_cycle = false;
        let mut i = 0;
        let mut y_before = 0;
        let mut rocks_before = 0;
        for jet in data.jet_pattern.iter().cycle() {
            // cycle loop stuff
            // if i == 0 {
            //     
            // }
            i += 1;
            if i == data.jet_pattern.len() {
                i = 0;

                cycle_num += 1;
                let dR = rock_count - rocks_before;
                let dY = (y_max - y_before) as u64;

                if cycle_num > CYCLES_BEFORE_STABLE && !did_cycle {

                    if cycle.get(0).eq(&Some(&(dY, dR))) { // Cycle established!! Skip ahead to end
                        // dbg!(&cycle);

                        let (mut tR, mut tY) = (0, 0);
                        for (dY, dR) in cycle.iter() {
                            tY += dY;
                            tR += dR;
                        }
                        let rocks_left = ROCK_LIMIT - rock_count;
                        let cycles_left = rocks_left / tR;
                        y_increase = tY * cycles_left;
                        let r_increase = tR * cycles_left;
                        rock_count += r_increase;
                        did_cycle = true;

                        // println!("{} {}, {} {} {} {}", tR, tY, rocks_left, cycles_left, y_increase, r_increase);
                    }
                    else {
                        cycle.push((dY, dR));
                    }
                }

                // println!("R: {}, dR: {}, Y: {}, dY: {}", rock_count, dR, y_max + y_increase, dY);

                y_before = y_max;
                rocks_before = rock_count;
            }

            match jet {
                Left => {
                    if pos.x != 0 {
                        pos.x -= 1;
                        if intersects(pos, rock, &mut blocks_set) {
                            pos.x += 1;
                        }
                    }
                }
                Right => {
                    if pos.x != 6 {
                        pos.x += 1;
                        if intersects(pos, rock, &mut blocks_set) {
                            pos.x -= 1;
                        }
                    }
                }
            }

            pos.y -= 1;
            if intersects(pos, rock, &mut blocks_set) || pos.y == 0 {
                pos.y += 1;
                rock_count += 1;

                y_max = insert_rock(pos, rock, y_max, &mut blocks_set);

                if rock_count == ROCK_LIMIT {
                    break;
                }

                rock = rocks.next().unwrap();
                pos = Vector2 {x: 2, y: y_max + 4};
            }
        }

        (y_max + y_increase) as usize
        // 0
        // solution(data, 1000000000000)
    }
}

fn insert_rock(pos: Vector2<u64>, rock: &Rock, mut y_max: u64, blocks_set: &mut HashSet<Vector2<u64>>) -> u64 {
    for square_pos in rock_to_squares(rock) {
        let new_pos = pos + square_pos;
        y_max = y_max.max(new_pos.y);
        blocks_set.insert(new_pos);
    }

    y_max
}

fn intersects(pos: Vector2<u64>, rock: &Rock, blocks_set: &mut HashSet<Vector2<u64>>) -> bool {
    for square_pos in rock_to_squares(rock) {
        let new_pos = pos + square_pos;
        if blocks_set.contains(&new_pos) {
            return true;
        }
        if new_pos.x >= 7 {
            return true;
        }
    }

    false
}

fn print_blocks(block_set: &HashSet<Vector2<u64>>, y_max: u64) {
    for y in (1..=y_max).rev() {
        println!("|{}|", (0..7).map(|x| {
            match block_set.contains(&Vector2{x, y}) {
                true => '#',
                false => '.'
            }
        }).collect::<String>());
    }
    println!("+-------+");
    println!();
}