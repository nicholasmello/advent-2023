use std::fs;

#[derive(Debug)]
struct AlmanacTranslationLayer {
    source: i64,
    destination: i64,
    size: i64,
}

impl AlmanacTranslationLayer {
    fn _parse_str(s: Option<&str>) -> i64 {
        s.expect("layer missing 3 numbers")
            .parse::<i64>()
            .expect("could not find number")
    }

    fn from_str(s: &str) -> Self {
        let mut numbers = s.trim()
            .split(" ");

        Self {
            destination: Self::_parse_str(numbers.nth(0)),
            source: Self::_parse_str(numbers.nth(0)),
            size: Self::_parse_str(numbers.nth(0)),
        }
    }
}

#[derive(Debug)]
struct AlmanacTranslator {
    descriptor: String,
    layers: Vec<AlmanacTranslationLayer>,
}

impl AlmanacTranslator {
    fn from_str(s: &str) -> Self {
        let (descriptor, lines) = s.split_once(":")
            .expect("no descriptor found");

        let descriptor = descriptor.to_string();

        let lines = lines.lines()
            .filter(|line| line.trim() != "");

        let layers = lines.map(|line| AlmanacTranslationLayer::from_str(line))
            .collect();

        Self {descriptor,layers}
    }

    fn apply_translation(&self, input: i64) -> i64 {
        println!("Applying layer: {}", self.descriptor);
        let mut output = input;

        for layer in &self.layers {
            if layer.source <= output && output < (layer.source + layer.size) {
                output = output + (layer.destination - layer.source);
                break;
            }
        }
        output
    }
}

fn apply_all_translations(translations: &Vec<AlmanacTranslator>, input: i64) -> i64 {
    let mut output = input;
    for translation in translations {
        output = translation.apply_translation(output);
    }
    output
}

fn main() {
    let file = fs::read_to_string("almanac.txt")
        .expect("cannot read file");
    let (seeds, translations) = file.split_once("\n\n").unwrap();
    let (_, seeds) = seeds.split_once(":").unwrap();
    let seeds: Vec<i64> = seeds.trim()
        .split(" ")
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    let translations: Vec<AlmanacTranslator> = translations.split("\n\n")
        .map(|s| AlmanacTranslator::from_str(s))
        .collect();

    let adjusted_seeds: Vec<i64> = seeds.into_iter()
        .map(|seed| apply_all_translations(&translations, seed))
        .collect();

    println!("{:?}", adjusted_seeds.into_iter().min().unwrap());
}
