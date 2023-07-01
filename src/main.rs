mod day;
mod day16;

use crate::day16::Day16;
use crate::day::run_with_test;

fn main() -> std::io::Result<()> {
    println!("Running Day 16:");
    
    test_day16()?;
    
    Ok(())
}

fn test_day16() -> std::io::Result<()> {
    run_with_test(&Day16, "day16e.txt", (1651, 1707), "day16.txt")
}
