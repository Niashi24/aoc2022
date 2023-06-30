mod day;
mod day16;

use crate::day16::Day16;
use crate::day::{Day, run};

fn main() -> std::io::Result<()> {
    println!("Running Day 16:");
    
    let day16 = Day16;
    run(&day16, "day16.txt")?;
    
    Ok(())
}