use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // print the cwd
    let cwd = std::env::current_dir()?;
    println!("Current working directory: {}", cwd.display());

    let file = File::open("day1.txt")?;
    let reader = io::BufReader::new(file);

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        let parts: Vec<&str> = line.splitn(2, "   ").collect();
        if parts.len() == 2 {
            left.push(parts[0].to_string().trim().parse::<i32>().unwrap());
            right.push(parts[1].trim().parse::<i32>().unwrap());
        }
    }

    // go through each number in left col and multiply by occurences in right col
    let mut result = 0;

    for v in left.iter() {
        let count = right.iter().filter(|&&ref x| x == v).count() as i32;
        result += v * count;
    }

    println!("Result: {}", result);

    Ok(())
}
