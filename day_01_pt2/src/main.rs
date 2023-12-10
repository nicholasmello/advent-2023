use std::fs;
use regex::{Regex, Captures, Replacer};

struct Parser;

impl Replacer for Parser {
    fn replace_append(&mut self, caps: &Captures, dst: &mut String) {
        match &caps["num"] {
            "one" => dst.push_str("o1e"),
            "two" => dst.push_str("t2o"),
            "three" => dst.push_str("t3e"),
            "four" => dst.push_str("f4r"),
            "five" => dst.push_str("f5e"),
            "six" => dst.push_str("s6x"),
            "seven" => dst.push_str("s7n"),
            "eight" => dst.push_str("e8t"),
            "nine" => dst.push_str("n9e"),
            _ => panic!("Regex got a little crazy and grabbed more than we want"),
        }
    }
}

fn parse_line(line: &str) -> Vec<i32> {
    let number_filter = Regex::new(r"[1-9]").unwrap();
    let text_filter = Regex::new(r"(?<num>(one|two|three|four|five|six|seven|eight|nine))").unwrap();

    // Replace text pass 1
    let result = text_filter.replace_all(line, Parser).into_owned();

    // Replace text pass 2
    let result = text_filter.replace_all(&result, Parser).into_owned();

    // Gather Integers
    number_filter.find_iter(&result)
        .map(|m| m.as_str().parse::<i32>().unwrap())
        .collect()
}

fn get_values(vec: &Vec<i32>) -> i32 {
    let last = vec.iter().last().unwrap();
    if vec.len() == 1 {
        return last + 10*last;
    } else {
        return last + 10*vec.into_iter().nth(0).unwrap();
    }
}

fn main() {
    // Filter words into numbers
    let file = "calibration_document.txt";
    let contents: Vec<Vec<i32>> = fs::read_to_string(file)
        .expect("Unable to read file")
        .lines()
        .map(|line| parse_line(line))
        .collect();

    // Get values based on first and last number
    let contents: Vec<i32> = contents.iter()
        .map(|line| get_values(line))
        .collect();

    // Calculate sum and return it
    let contents:i32 = contents.iter().sum();
    println!("{contents}");
}
