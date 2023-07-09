use std::fs::{self, read_to_string};

pub trait Day<TData> {
    fn parse_file(&self, file_content: String) -> TData;

    fn part_1(&self, data: &TData) -> i64;

    fn part_2(&self, data: &TData) -> i64;
}

pub fn run_with_test<TData, TDay: Day<TData>>(
    day: &TDay,
    example_file: &str,
    example_expected: (i64, i64),
    full_file: &str,
) -> std::io::Result<()> {
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

pub fn run_with_test_2<TData, TDay: Day<TData>>(
    day: &TDay,
    example_file: &str,
    example_expected: (i64, i64),
    full_file: &str,
) -> std::io::Result<()> {
    let file_content_example = read_to_string(example_file)?;
    let file_content_full = read_to_string(full_file)?;

    print!("Example - ");
    let file_data_example = test_parse_data(day, file_content_example);
    print!("Full - ");
    let file_data_full = test_parse_data(day, file_content_full);

    print!("Example - ");
    let part_1_example = test_part_1(day, &file_data_example);
    if part_1_example != example_expected.0 {
        println!(
            "Error! Expected answer\n\"Part 1: {}\", but got\n\"Part 1: {}\"\x07",
            example_expected.0, part_1_example
        );
        return Ok(());
    }

    print!("Actual - ");
    let _ = test_part_1(day, &file_data_full);

    print!("Example - ");
    let part_2_example = test_part_2(day, &file_data_example);
    if part_2_example != example_expected.1 {
        println!(
            "Error! Expected answer\n\"Part 2: {}\", but got\n\"Part 2: {}\"\x07",
            example_expected.1, part_2_example
        );
        return Ok(());
    }

    print!("Actual - ");
    let _ = test_part_2(day, &file_data_full);

    print!("\x07");

    Ok(())
}

fn test_part_1<TData, TDay: Day<TData>>(day: &TDay, data: &TData) -> i64 {
    let now = std::time::Instant::now();
    let part_1 = day.part_1(data);
    let elapsed = now.elapsed();
    println!("Part 1: {}", part_1);
    println!("Elapsed Time: {:.2?}", elapsed);
    println!();

    part_1
}

fn test_part_2<TData, TDay: Day<TData>>(day: &TDay, data: &TData) -> i64 {
    let now = std::time::Instant::now();
    let part_2 = day.part_2(data);
    let elapsed = now.elapsed();
    println!("Part 2: {}", part_2);
    println!("Elapsed Time: {:.2?}", elapsed);
    println!();

    part_2
}

fn test_parse_data<TData, TDay: Day<TData>>(day: &TDay, file_content: String) -> TData {
    let now = std::time::Instant::now();
    let file_data = day.parse_file(file_content);
    let elapsed = now.elapsed();
    println!("Parsed file.");
    println!("Elapsed Time: {:.2?}", elapsed);
    println!();

    file_data
}

pub fn run<TData, TDay: Day<TData>>(day: &TDay, file_name: &str) -> std::io::Result<(i64, i64)> {
    let file_content = fs::read_to_string(file_name)?;

    let file_data = test_parse_data(day, file_content);

    let part_1 = test_part_1(day, &file_data);
    let part_2 = test_part_2(day, &file_data);

    Ok((part_1, part_2))
}
