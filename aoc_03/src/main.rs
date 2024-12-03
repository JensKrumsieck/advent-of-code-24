use std::{
    error::Error,
    fs::{self},
};

use regex::Regex;

fn main() {
    let data = read_file(&format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "input.txt")).unwrap();
    println!("fixed multiplication: {}", multipy(&data));
    println!("fixed do/dont multiplication: {}", multipy_do_dont(&data));
}

fn multipy(input: &[String]) -> i32 {
    let regex = Regex::new(r#"mul\((\d+),(\d+)\)"#).unwrap();
    let mut sum = 0;

    for line in input {
        for item in regex.captures_iter(line) {
            let first = item
                .get(1)
                .unwrap()
                .as_str()
                .parse::<i32>()
                .unwrap_or_default();
            let second = item
                .get(2)
                .unwrap()
                .as_str()
                .parse::<i32>()
                .unwrap_or_default();
            sum += first * second;
        }
    }

    sum
}

fn multipy_do_dont(input: &[String]) -> i32 {
    let mut enabled = true;
    let mut sum = 0;

    let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();

    for line in input {
        let mut pos = 0;
        
        while pos < line.len() {
            if let Some(mul_caps) = mul_re.captures(&line[pos..]) {
                let a: i32 = mul_caps[1].parse().unwrap();
                let b: i32 = mul_caps[2].parse().unwrap();

                if enabled {
                    sum += a * b;
                }

                pos += mul_caps.get(0).unwrap().end();
            } else if let Some(_) = do_re.find(&line[pos..]) {
                enabled = true;
                pos += 4;
            } else if let Some(_) = dont_re.find(&line[pos..]) {
                enabled = false;
                pos += 8;
            } else {
                pos += 1;
            }
        }
    }

    sum
}

fn read_file(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let lines = contents.lines().map(|i| i.to_string()).collect::<Vec<_>>();
    Ok(lines)
}
