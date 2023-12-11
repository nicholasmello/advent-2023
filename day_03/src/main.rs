use std::fs;
use core::ops::Range;
use regex::Regex;

struct SchematicNumber {
    row: usize,
    column: Range<usize>,
}

fn parse_to_schematic_number(vecs: Vec<Vec<Range<usize>>>) -> Vec<SchematicNumber> {
    let mut ret: Vec<SchematicNumber> = vec![];
    for (i, vec) in vecs.into_iter().enumerate() {
        for range in vec {
            ret.push(SchematicNumber {
                row: i,
                column: range,
            });
        }
    }
    ret
}

fn check_square_for_symbols(schematic: &Vec<Vec<char>>, row: &usize, col: &usize) -> bool {
    let filter_symbols = Regex::new(r"[^0-9|.|\n]").unwrap();

    let character = match schematic.iter().nth(*row) {
        None => return false,
        Some(c) => c,
    };
    let character = match character.iter().nth(*col) {
        None => return false,
        Some(c) => c,
    };

    filter_symbols.is_match(&character.to_string())
}

fn check_schematic_for_symbols(schematic: &Vec<Vec<char>>, row: &usize, col: &usize) -> bool {
    for i in -1..2 {
        for j in -1..2 {
            let row_to_check = match row.checked_add_signed(i) {
                None => continue,
                Some(val) => val,
            };
            let col_to_check = match col.checked_add_signed(j) {
                None => continue,
                Some(val) => val,
            };

            if check_square_for_symbols(&schematic, &row_to_check, &col_to_check) {
                return true;
            }
        }
    }
    return false;
}

fn main() {
    // Filters
    let filter_numbers = Regex::new(r"\d+").unwrap();

    // Schematic Parsing
    let file = "schematic.txt";
    let schematic_as_string = fs::read_to_string(file)
        .expect("Unable to read file");
    let schematic: Vec<Vec<char>> = schematic_as_string.lines()
        .map(|line| line.chars().collect())
        .collect();
    let numbers: Vec<Vec<Range<usize>>> = schematic_as_string.lines()
        .map(|line| filter_numbers.find_iter(line)
             .map(|m| m.range())
             .collect())
        .collect();
    let numbers = parse_to_schematic_number(numbers);

    // Check for symbols around numbers
    let mut numbers_with_symbol: Vec<String> = vec![];
    for number in numbers {
        let mut contains_symbol = false;
        for col in number.column.clone() {
            if check_schematic_for_symbols(&schematic, &number.row, &col) {
                contains_symbol = true;
                break;
            }
        }
        if contains_symbol {
            numbers_with_symbol.push(schematic[number.row][number.column.start..number.column.end].iter().collect());
        }
    }

    println!("{}", numbers_with_symbol.into_iter().map(|s| s.parse::<u32>().unwrap()).sum::<u32>());
}
