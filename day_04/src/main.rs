use std::fs;
use regex::Regex;

struct Card {
    number: u32,
    winning: Vec<u32>,
    scratched: Vec<u32>,
}

enum CardNumber {
    PIPE,
    NUMBER(u32),
}

fn parse_to_card(line: &str) -> Card {
    let mut game_number = None;
    let mut winning_cards = vec![];
    let mut scratched_cards = vec![];

    let filter = Regex::new(r"\d+|\|").unwrap();
    let numbers: Vec<CardNumber> = filter.find_iter(line)
        .map(|m|
             if m.as_str() == "|" {
                 return CardNumber::PIPE;
             } else {
                 return CardNumber::NUMBER(m.as_str().parse::<u32>().unwrap());
             })
        .collect();

    let mut seen_divider = false;
    for (i, number) in numbers.into_iter().enumerate() {
        match number {
            CardNumber::PIPE => seen_divider = true,
            CardNumber::NUMBER(num) => {
                if i == 0 {
                    game_number = Some(num);
                    continue;
                }

                if seen_divider {
                    scratched_cards.push(num);
                } else {
                    winning_cards.push(num);
                }
            }
        }
    }

    Card {
        number: game_number.unwrap(),
        winning: winning_cards,
        scratched: scratched_cards,
    }
}

fn main() {
    let file = "cards.txt";
    let cards: Vec<Card> = fs::read_to_string(file)
        .expect("unable to read file")
        .lines()
        .map(|line| parse_to_card(line))
        .collect();

    let mut sum = 0;
    for card in cards {
        let mut number_of_matched = 0;
        for num in card.winning {
            if card.scratched.contains(&num) {
                number_of_matched += 1;
            }
        }

        if number_of_matched == 0 {
            continue;
        } else {
            // Bitshift 2^x
            sum += 1 << (number_of_matched-1);
        }
    }

    println!("{sum}");
}
