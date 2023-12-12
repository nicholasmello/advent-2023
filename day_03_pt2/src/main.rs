use std::fs;
use core::ops::Range;
use regex::Regex;

struct SchematicNumber {
    row: usize,
    column: Range<usize>,
}

fn parse_to_schematic_number(vecs: &Vec<Vec<Range<usize>>>) -> Vec<SchematicNumber> {
    let mut ret: Vec<SchematicNumber> = vec![];
    for (i, vec) in vecs.into_iter().enumerate() {
        for range in vec {
            ret.push(SchematicNumber {
                row: i,
                column: range.clone(),
            });
        }
    }
    ret
}

struct GearLocation {
    row: usize,
    column: usize,
}

fn parse_to_location(vecs: Vec<Vec<usize>>) -> Vec<GearLocation> {
    let mut ret: Vec<GearLocation> = vec![];
    for (i, vec) in vecs.into_iter().enumerate() {
        for range in vec {
            ret.push(GearLocation {
                row: i,
                column: range,
            });
        }
    }
    ret
}

fn main() {
    // Filters
    let filter_numbers = Regex::new(r"\d+").unwrap();
    let filter_gears = Regex::new(r"\*").unwrap();

    // Schematic Parsing
    let file = "schematic.txt";
    let schematic_as_string = fs::read_to_string(file)
        .expect("Unable to read file");
    let schematic: Vec<Vec<char>> = schematic_as_string.lines()
        .map(|line| line.chars().collect())
        .collect();
    let gears: Vec<Vec<usize>> = schematic_as_string.lines()
        .map(|line| filter_gears.find_iter(line)
             .map(|m| m.range().start)  // Start and end are same value of regex is working
                                        // properly
            .collect())
        .collect();
    let gears = parse_to_location(gears);
    let numbers: Vec<Vec<Range<usize>>> = schematic_as_string.lines()
        .map(|line| filter_numbers.find_iter(line)
             .map(|m| m.range())
             .collect())
        .collect();
    let numbers = parse_to_schematic_number(&numbers);

    // Look for numbers around 'gears'
    let mut sum = 0;
    for gear in gears {
        let mut adjacent_numbers: Vec<u32> = vec![];
        for number in &numbers {
            let mut double_break = false;
            for i in -1..2 {
                for j in -1..2 {
                    let row_to_check = match gear.row.checked_add_signed(i) {
                        None => continue,
                        Some(val) => val,
                    };
                    let col_to_check = match gear.column.checked_add_signed(j) {
                        None => continue,
                        Some(val) => val,
                    };

                    if number.row != row_to_check {
                        continue;
                    }

                    if number.column.start <= col_to_check && col_to_check < number.column.end {
                        let num: String = schematic[number.row][number.column.start..number.column.end].into_iter().collect();
                        adjacent_numbers.push(num.parse::<u32>().unwrap());
                        double_break = true;
                        break;
                    }
                }
                if double_break {
                    break;
                }
            }
        }
        if adjacent_numbers.len() == 2 {
            sum += adjacent_numbers[0] * adjacent_numbers[1];
        }
    }

    println!("{sum}");
}
