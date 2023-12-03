use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::Result;

fn main() {
    let input = Path::new("./input");

    let sum = get_file_sum(input, false);

    match sum {
        Ok(sum) => {
            println!("File sum: {}", sum);
        }
        Err(_) => eprintln!("Error getting file sum"),
    }

    let input = Path::new("./input2");

    let sum = get_file_sum(input, true);

    match sum {
        Ok(sum) => {
            println!("File2 sum: {}", sum);
        }
        Err(_) => eprintln!("Error getting file2 sum"),
    }
}

fn patch_line(line: &str) -> String {
    const REPLACEMENTS: [(&str, &str); 9] = [
        ("one", "one1one"),
        ("two", "two2two"),
        ("three", "three3three"),
        ("four", "four4four"),
        ("five", "five5five"),
        ("six", "six6six"),
        ("seven", "seven7seven"),
        ("eight", "eight8eight"),
        ("nine", "nine9nine"),
    ];

    let mut new_line = String::from(line);

    for tuple in REPLACEMENTS {
        new_line = new_line.replace(tuple.0, tuple.1);
    }

    new_line
}

fn get_file_sum(path: &Path, patch_lines: bool) -> Result<u64> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {
        let Ok(mut line) = line else {
            eprintln!("Line was not readable!");
            continue;
        };

        if patch_lines {
            line = patch_line(&line);
        }

        let line_number = get_number(&line)?;

        sum += line_number;
    }

    Ok(sum)
}

fn get_number(line: &str) -> Result<u64> {
    let chars: Vec<char> = line.chars().collect();

    let mut number_string = String::from("");

    for char in chars.iter() {
        if char.is_numeric() {
            number_string.push(*char);
            break;
        }
    }

    for char in chars.iter().rev() {
        if char.is_numeric() {
            number_string.push(*char);
            break;
        }
    }

    let number: u64 = number_string.parse()?;

    Ok(number)
}
