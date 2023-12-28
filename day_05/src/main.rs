use std::fs;
use std::ops::Range;

#[derive(Clone)]
struct AlmanacSeed {
    seeds: Vec<Range<i64>>,
}

impl AlmanacSeed {
    fn apply_translations(&self, translations: &Vec<AlmanacTranslator>) -> Self {
        let mut output = self.clone();
        for translation in translations {
            output = translation.apply_translation(&output);
        }
        output
    }
}


#[derive(Debug)]
struct AlmanacTranslationLayer {
    source: Range<i64>,
    destination: i64,
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

        let start = Self::_parse_str(numbers.nth(0));

        Self {
            destination: Self::_parse_str(numbers.nth(0)),
            source: Range {
                start,
                end: start + Self::_parse_str(numbers.nth(0)),
            }
        }
    }
}

#[derive(Debug)]
struct AlmanacTranslator {
    _descriptor: String,
    layers: Vec<AlmanacTranslationLayer>,
}

impl AlmanacTranslator {
    fn from_str(s: &str) -> Self {
        let (descriptor, lines) = s.split_once(":")
            .expect("no descriptor found");

        let _descriptor = descriptor.to_string();

        let lines = lines.lines()
            .filter(|line| line.trim() != "");

        let layers = lines.map(|line| AlmanacTranslationLayer::from_str(line))
            .collect();

        Self {_descriptor,layers}
    }

    fn apply_translation(&self, input: &AlmanacSeed) -> AlmanacSeed {
        println!("DEBUG: Applying layer: {}", self._descriptor);
        let mut output = vec![];

        for seed in &input.seeds {
            for layer in &self.layers {
                let offset = layer.destination - layer.source.start;

                if layer.source.contains(&seed.start) && layer.source.contains(&seed.end) {
                    // Move all
                    output.push(Range {
                        start: seed.start + offset,
                        end: seed.end + offset,
                    });
                } else if seed.contains(&layer.source.start) && seed.contains(&layer.source.end) {
                    // Cutout
                    output.push(Range {
                        start: layer.source.start + offset,
                        end: layer.source.end + offset,
                    });

                    // Keep right
                    output.push(Range {
                        start: layer.source.end,
                        end: seed.end,
                    });

                    // Keep left
                    output.push(Range {
                        start: seed.start,
                        end: layer.source.start,
                    });
                } else if layer.source.contains(&seed.start) && seed.contains(&layer.source.end) {
                    // Move left
                    output.push(Range {
                        start: seed.start + offset,
                        end: layer.source.end + offset,
                    });
                    // Keep right
                    output.push(Range {
                        start: layer.source.start,
                        end: seed.end,
                    });
                } else if seed.contains(&layer.source.start) && layer.source.contains(&seed.end) {
                    // Keep left
                    output.push(Range {
                        start: seed.start,
                        end: layer.source.end,
                    });
                    // Move right
                    output.push(Range {
                        start: layer.source.start + offset,
                        end: seed.end + offset,
                    });
                    todo!();
                } else {
                    // no overlap, move nothing
                    output.push(seed.clone());
                }
            }
        }

        AlmanacSeed {seeds: output}
    }
}

fn part_01(seeds: &Vec<i64>, translations: &Vec<AlmanacTranslator>) -> i64 {
    let seeds: Vec<Range<i64>> = seeds.clone()
        .into_iter()
        .map(|seed| Range {start: seed, end: seed})
        .collect();
    let seeds = AlmanacSeed{seeds};
    let seeds = seeds.apply_translations(translations);
    seeds.seeds
        .into_iter()
        .map(|seed| seed.start)
        .min()
        .unwrap()
}

fn part_02(seeds: &Vec<i64>, translations: &Vec<AlmanacTranslator>) -> i64 {
    let mut seed_ranges: Vec<Range<i64>> = vec![];
    let mut current_seed = None;
    for seed in seeds {
        match current_seed {
            None => current_seed = Some(seed),
            Some(s) => {
                seed_ranges .push(Range {
                    start: *s,
                    end: *seed,
                });
                current_seed = None;
            }
        };
    }

    let seeds = AlmanacSeed{seeds: seed_ranges};
    let seeds = seeds.apply_translations(translations);
    seeds.seeds
        .into_iter()
        .map(|seed| seed.start)
        .min()
        .unwrap()
}

fn main() {
    // Read from file and parse data into structs
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

    // Solution to Part 1
    let part_01 = part_01(&seeds, &translations);

    // Solution to Part 2
    let part_02 = part_02(&seeds, &translations);

    println!("\n-----------------------------");
    println!("Solution to part 1: {part_01}");
    println!("Solution to part 2: {part_02}");
    println!("-----------------------------");
}
