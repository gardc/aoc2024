use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("day2.txt").unwrap();
    let reader = io::BufReader::new(file);

    let safe_reports = reader
        .lines()
        .map(|line| {
            let numbers: Vec<i32> = line
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            report_is_safe(&numbers)
        })
        .filter(|&is_safe| is_safe)
        .count();

    println!("Reports: {}", safe_reports);
}

fn report_is_safe(numbers: &[i32]) -> bool {
    if numbers.len() < 2 {
        return true;
    }

    let direction = numbers[1] > numbers[0];
    let direction_value = if direction { 1 } else { -1 };

    numbers.windows(2).all(|window| {
        let (prev, current) = (window[0], window[1]);

        if (current > prev) != direction {
            return false;
        }

        let max_allowed = prev + 3 * direction_value;
        let min_allowed = prev + 1 * direction_value;
        let (lower, upper) = if direction {
            (min_allowed, max_allowed)
        } else {
            (max_allowed, min_allowed)
        };

        (lower..=upper).contains(&current)
    })
}
