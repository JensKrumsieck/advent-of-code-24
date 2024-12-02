use std::{error::Error, fs};

fn main() {
    let data = read_file(&format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "input.txt"))
        .expect("Could not read input file!");
    println!("Number of safe lines {}", count_safe_lines(&data));
    println!(
        "Number of safe lines with alternative {}",
        count_safe_lines_alt(&data)
    );
}

fn count_safe_lines(data: &Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    for line in data {
        if is_safe(line) {
            count += 1;
        }
    }
    count
}

fn count_safe_lines_alt(data: &Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    for line in data {
        if is_safe(line) || is_safe_alt(line) {
            count += 1;
        }
    }
    count
}

fn is_safe_alt(levels: &[i32]) -> bool {
    for i in 0..levels.len() {
        let variation = levels
            .iter()
            .enumerate()
            .filter(|&(x, _)| x != i)
            .map(|(_, item)| *item)
            .collect::<Vec<_>>();
        if is_safe(&variation) {
            return true;
        }
    }
    false
}

fn is_safe(levels: &[i32]) -> bool {
    if levels.len() < 2 {
        return true;
    }

    let mut is_increasing = None;

    for window in levels.windows(2) {
        let diff = window[1] - window[0];

        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }

        if let Some(increasing) = is_increasing {
            if increasing && diff < 0 {
                return false; // Was increasing, now decreasing.
            } else if !increasing && diff > 0 {
                return false; // Was decreasing, now increasing.
            }
        } else {
            is_increasing = Some(diff > 0);
        }
    }

    true
}

fn read_file(filename: &str) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let lines = contents
        .lines()
        .map(|str| {
            str.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap_or_default())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    Ok(lines)
}
