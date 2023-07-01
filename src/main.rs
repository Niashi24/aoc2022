mod day;
mod day16;

use crate::day16::Day16;
use crate::day::{Day, run, run_with_test};

fn main() -> std::io::Result<()> {
    println!("Running Day 16:");
    
    let day16 = Day16;
    run_with_test(&day16, "day16e.txt", (1651, 1707), "day16.txt")?;
    
    Ok(())
}