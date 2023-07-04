mod day;
mod day16;
mod day18;

use crate::day16::Day16;
use crate::day18::Day18;
use crate::day::run_with_test;

fn main() -> std::io::Result<()> {
    
    test_day16()?;
    test_day18()?;
    
    Ok(())
}

fn test_day18() -> std::io::Result<()> {
    println!("Running Day 18:");
    run_with_test(&Day18, "input/day18e.txt", (64, 58), "input/day18.txt")
}

fn test_day16() -> std::io::Result<()> {
    println!("Running Day 16:");
    run_with_test(&Day16, "input/day16e.txt", (1651, 1707), "input/day16.txt")
}
