use std::fs;
use regex::Regex;

struct ScratchCard {
    winning: Vec<u32>,
    scratched: Vec<u32>,
}

impl ScratchCard {
    fn from_str(s: &str) -> Self {
        // Filter for any number of numbers in a row
        let filter = Regex::new(r"\d+").unwrap();

        // Remove Card Header
        let card = s.split(":")
            .nth(1)
            .expect("Card Header not found");

        // Split by divider
        let (winning, scratched) = card.split_once("|").unwrap();

        // Assemble ScratchCard
        let winning = filter.find_iter(winning)
            .map(|m| m.as_str().parse::<u32>().unwrap())
            .collect();
        let scratched = filter.find_iter(scratched)
            .map(|m| m.as_str().parse::<u32>().unwrap())
            .collect();
        Self {
            winning,
            scratched,
        }
    }

    fn matches(&self) -> Vec<u32> {
        let mut matches = vec![];
        for card in &self.winning {
            if self.scratched.contains(&card) {
                matches.push(*card);
            }
        }
        matches
    }
}

fn main() {
    // Get cards from text file
    let file = fs::read_to_string("cards.txt")
        .expect("cannot read file");
    let cards: Vec<ScratchCard> = file.lines()
        .into_iter()
        .map(|line| ScratchCard::from_str(line))
        .collect();

    // Calculate card values
    let values: Vec<usize> = cards.into_iter()
        .map(|c| c.matches().len())
        .collect();

    // Calculate number of cards after clones
    let mut multipliers = vec![1; values.len()];
    let mut sum = 0;
    for (i, value) in values.into_iter().enumerate() {
        sum += 1*multipliers[i];

        for j in 0..value {
            multipliers[i+j+1] += multipliers[i];
        }
    }

    println!("{sum}");
}
