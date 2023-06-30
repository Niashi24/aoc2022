use std::fs;

pub trait Day<TData> {
    fn parse_file(&self, file_content: String) -> TData;
    
    fn part_1(&self, data: &TData) -> usize;
    
    fn part_2(&self, data: &TData) -> usize;
}

pub fn run<TData, TDay: Day<TData>>(day: &TDay, file_name: &str) -> std::io::Result<()> {
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

    Ok(())
}

impl<TData> dyn Day<TData> {
    pub fn run(&self, file_name: &str) -> std::io::Result<()> {
        let file_content = fs::read_to_string(file_name)?;
        
        use std::time::Instant;
        
        let now = Instant::now();
        let file_data = self.parse_file(file_content);
        let elapsed = now.elapsed();
        println!("Parsed file.");
        println!("Elapsed Time: {:.2?}", elapsed);
        println!();
        
        let now = Instant::now();
        let part_1 = self.part_1(&file_data);
        let elapsed = now.elapsed();
        println!("Part 1: {}", part_1);
        println!("Elapsed Time: {:.2?}", elapsed);
        println!();
        
        let now = Instant::now();
        let part_2 = self.part_2(&file_data);
        let elapsed = now.elapsed();
        println!("Part 2: {}", part_2);
        println!("Elapsed Time: {:.2?}", elapsed);
        println!();
        
        Ok(())
    }
}