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

const IHOR_ARR: [Vector2<u32>; 4] = [
    Vector2 {x: 0, y: 0},
    Vector2 {x: 1, y: 0},
    Vector2 {x: 2, y: 0},
    Vector2 {x: 3, y: 0},
];

const X_ARR: [Vector2<u32>; 4] = [
    Vector2 {x: 0, y: 1},
    Vector2 {x: 1, y: 0},
    Vector2 {x: 2, y: 1},
    Vector2 {x: 1, y: 2},
    //Vector2 {x: 1, y: 1},  // middle square is covered by other squares
];

const L_ARR: [Vector2<u32>; 5] = [
    Vector2 {x: 0, y: 0},
    Vector2 {x: 1, y: 0},
    Vector2 {x: 2, y: 0},
    Vector2 {x: 2, y: 1},
    Vector2 {x: 2, y: 2},
];

const IVERT_ARR: [Vector2<u32>; 4] = [
    Vector2 {x: 0, y: 0},
    Vector2 {x: 0, y: 1},
    Vector2 {x: 0, y: 2},
    Vector2 {x: 0, y: 3},
];

const O_ARR: [Vector2<u32>; 4] = [
    Vector2 {x: 0, y: 0},
    Vector2 {x: 1, y: 0},
    Vector2 {x: 0, y: 1},
    Vector2 {x: 1, y: 1},
];

fn rock_to_squares(rock: &Rock) -> core::slice::Iter<Vector2<u32>> {
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
        
        solution(data, 2022)
    }

    fn part_2(&self, data: &Info) -> usize {
        0
    }
}

fn solution(data: &Info, block_limit: u32) -> usize {
    let mut blocks_set: HashSet<Vector2<u32>> = HashSet::new();
    let mut y_max = 0u32;

    fn insert_rock(pos: Vector2<u32>, rock: &Rock, mut y_max: u32, blocks_set: &mut HashSet<Vector2<u32>>) -> u32 {
        for square_pos in rock_to_squares(rock) {
            let new_pos = pos + square_pos;
            y_max = y_max.max(new_pos.y);
            blocks_set.insert(new_pos);
        }

        y_max
    }

    fn intersects(pos: Vector2<u32>, rock: &Rock, blocks_set: &mut HashSet<Vector2<u32>>) -> bool {
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

            if rock_count == block_limit {
                break;
            }

            rock = rocks.next().unwrap();
            pos = Vector2 {x: 2, y: y_max + 4};
        }
    }

    y_max as usize
}

fn print_blocks(block_set: &HashSet<Vector2<u32>>, y_max: u32) {
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