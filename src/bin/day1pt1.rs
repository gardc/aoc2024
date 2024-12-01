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
            right.push(parts[1].to_string().trim().parse::<i32>().unwrap());
        }
    }

    // println!("Left column: {:?}", left);
    // println!("Right column: {:?}", right);

    // // sum all numbers in left and right columns

    // let mut sum_left = 0;
    // let mut sum_right = 0;

    // for num in left.iter() {
    //     sum_left += num.parse::<i32>().unwrap();
    // }
    // for num in right.iter() {
    //     sum_right += num.parse::<i32>().unwrap();
    // }

    // println!("Sum of left column: {}", sum_left);
    // println!("Sum of right column: {}", sum_right);

    // sort the left and right columns
    left.sort();
    right.sort();

    println!("Sorted left column: {:?}", &left[0..10.min(left.len())]);
    println!("Sorted right column: {:?}", &right[0..10.min(right.len())]);

    // find distance between left and right columns by iterating through both columns and doing abs(a - b)
    let mut distances = Vec::new();

    for (l, r) in left.iter().zip(right.iter()) {
        distances.push((l - r).abs());
    }

    println!("Distances between columns: {:?}", distances);

    // sum the distances
    let total_distance: i32 = distances.iter().sum();
    println!("Total distance: {}", total_distance);

    Ok(())
}
