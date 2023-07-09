mod day;
mod day16;
mod day17;
mod day18;
mod day20;

use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::day16::Day16;
use crate::day17::Day17;
use crate::day18::Day18;
use crate::day20::Day20;
use crate::day::run_with_test;


fn main() -> std::io::Result<()> {
    
    // test_day16()?;
    // test_day17()?;
    test_day20()?;
    
    // let max = 3;
    // let mut arr = (-max..=max).collect::<Vec<i32>>();
    // let mut rng = thread_rng();
    // arr.shuffle(&mut rng);
    // dbg!(arr);
    
    Ok(())
}

fn test_day16() -> std::io::Result<()> {
    println!("<--------    Running Day 16    -------->");
    run_with_test(&Day16, "input/day16e.txt", (1651, 1707), "input/day16.txt")
}

fn test_day17() -> std::io::Result<()> {
    println!("<--------    Running Day 17    -------->");
    run_with_test(&Day17, "input/day17e.txt", (3068, 1514285714288), "input/day17.txt")
}

fn test_day18() -> std::io::Result<()> {
    println!("<--------    Running Day 18    -------->");
    run_with_test(&Day18, "input/day18e.txt", (64, 58), "input/day18.txt")
}

fn test_day20() -> std::io::Result<()> {
    println!("<--------    Running Day 20    -------->");
    run_with_test(&Day20, "input/day20e.txt", (3, 1623178306), "input/day20.txt")
}