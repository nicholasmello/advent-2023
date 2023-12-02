use std::fs;
use regex::Regex;

fn get_values(vec: &Vec<i32>) -> i32 {
    let last = vec.iter().last().unwrap();
    if vec.len() == 1 {
        return last + 10*last;
    } else {
        return last + 10*vec.into_iter().nth(0).unwrap();
    }
}

fn main() {
    // Regex for numbers
    let filter = Regex::new(r"[1-9]").unwrap();

    // Parse file for numbers into vec of vec
    let file = "calibration_document.txt";
    let contents: Vec<Vec<i32>> = fs::read_to_string(file)
        .expect("Unable to read file")
        .lines()
        .map(|line| filter.find_iter(line)
            .map(|m| m.as_str().parse::<i32>().unwrap())
            .collect())
        .collect();

    // Get values based on first and last number
    let contents: Vec<i32> = contents.iter()
        .map(|line| get_values(line))
        .collect();

    // Calculate sum and return it
    let contents:i32 = contents.iter().sum();
    println!("{contents}")
}
