use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::{self},
};

fn main() {
    let mut data = read_file(&format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "input.txt"))
        .expect("Could not read input file!");
    data.0.sort();
    data.1.sort();

    let sum = calc_difference(&data);
    let sim = calc_similarity(&data);

    println!("Difference: {:?}", sum);
    println!("Similarity: {:?}", sim);
}

fn calc_similarity(sorted_data: &(Vec<i32>, Vec<i32>)) -> i32 {
    let mut counts = HashMap::new();
    for &number in &sorted_data.1 {
        *counts.entry(number).or_insert(0) += 1;
    }

    let mut similarity = 0;
    let mut seen = HashSet::new();
    for &number in &sorted_data.0 {
        if seen.insert(number) {
            similarity += number * counts.get(&number).copied().unwrap_or(0);
        }
    }

    similarity
}

fn calc_difference(sorted_data: &(Vec<i32>, Vec<i32>)) -> i32 {
    let mut diff = vec![];
    for i in 0..sorted_data.0.len() {
        diff.push((sorted_data.0[i] - sorted_data.1[i]).abs());
    }

    let sum: i32 = diff.into_iter().sum();
    sum
}

fn read_file(filename: &str) -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let lines = contents.lines();

    let mut left = Vec::with_capacity(lines.clone().count());
    let mut right = Vec::with_capacity(lines.clone().count());

    for line in lines {
        let mut items = line.split_whitespace();
        let left_str = items.next().ok_or("Missing left value")?;
        let right_str = items.next().ok_or("Missing right value")?;

        left.push(left_str.parse::<i32>()?);
        right.push(right_str.parse::<i32>()?);
    }

    Ok((left, right))
}
