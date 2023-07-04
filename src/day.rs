use std::fs;

pub trait Day<TData> {
    fn parse_file(&self, file_content: String) -> TData;
    
    fn part_1(&self, data: &TData) -> usize;
    
    fn part_2(&self, data: &TData) -> usize;
}

pub fn run_with_test<TData, TDay: Day<TData>>(day: &TDay, example_file: &str, example_expected: (usize, usize), full_file: &str) -> std::io::Result<()> {
    println!("Testing with example dataset: ");
    let example_actual = run(day, example_file)?;
    if example_actual != example_expected {
        println!("Error! Expected answer\n\"Part 1: {}, Part 2: {}\", but got\n\"Part 1: {}, Part 2: {}\"\x07", 
                 example_expected.0, example_expected.1,
                 example_actual.0, example_actual.1);
        
        return Ok(());
    }
    
    println!("Example Successful! Moving to full dataset:");
    let _ = run(day, full_file)?;
    
    print!("\x07");
    
    Ok(())
}

pub fn run<TData, TDay: Day<TData>>(day: &TDay, file_name: &str) -> std::io::Result<(usize, usize)> {
    let file_content = fs::read_to_string(file_name)?;

    use std::time::Instant;

    let now = Instant::now();
    let file_data = day.parse_file(file_content);
    let elapsed = now.elapsed();
    println!("Parsed file.");
    println!("Elapsed Time: {:.2?}", elapsed);
    println!();

    let now = Instant::now();
    let part_1 = day.part_1(&file_data);
    let elapsed = now.elapsed();
    println!("Part 1: {}", part_1);
    println!("Elapsed Time: {:.2?}", elapsed);
    println!();

    let now = Instant::now();
    let part_2 = day.part_2(&file_data);
    let elapsed = now.elapsed();
    println!("Part 2: {}", part_2);
    println!("Elapsed Time: {:.2?}", elapsed);
    println!();

    Ok((part_1, part_2))
}