use std::fs;
use regex::Regex;

struct GameCubes {
    red: u32,
    green: u32,
    blue: u32,
}

struct GameInfo {
    number: u32,
    cubes: GameCubes,
    valid: bool,
}

const MAX_CUBE: GameCubes =  GameCubes{
    red: 12,
    green: 13,
    blue: 14,
};

fn parse_line(line: &str) -> GameInfo {
    // Setup filters
    // Backtracing not allowed so removing fixed offset
    let filter_number = Regex::new(r"Game \d*").unwrap();
    let filter_red = Regex::new(r"\d* red").unwrap();
    let filter_green = Regex::new(r"\d* green").unwrap();
    let filter_blue = Regex::new(r"\d* blue").unwrap();

    // Get game number
    let game_number = filter_number.find(line)
        .expect("Game to have number")
        .as_str()[5..]
        .parse::<u32>()
        .unwrap();

    // Fill out cubes
    let mut biggest_cubes = GameCubes {red: 0, blue: 0, green: 0};
    let reds: Vec<u32> = filter_red.find_iter(line)
        .map(|s| s.as_str().strip_suffix(" red").unwrap().parse::<u32>().unwrap())
        .collect();
    for red in reds {
        if red > biggest_cubes.red {
            biggest_cubes.red = red;
        }
    }
    let greens: Vec<u32> = filter_green.find_iter(line)
        .map(|s| s.as_str().strip_suffix(" green").unwrap().parse::<u32>().unwrap())
        .collect();
    for green in greens {
        if green > biggest_cubes.green {
            biggest_cubes.green = green;
        }
    }
    let blues: Vec<u32> = filter_blue.find_iter(line)
        .map(|s| s.as_str().strip_suffix(" blue").unwrap().parse::<u32>().unwrap())
        .collect();
    for blue in blues {
        if blue > biggest_cubes.blue {
            biggest_cubes.blue = blue;
        }
    }

    // Determine cube validity
    let mut is_valid = true;
    if biggest_cubes.red > MAX_CUBE.red {
        is_valid = false;
    }
    if biggest_cubes.green > MAX_CUBE.green {
        is_valid = false;
    }
    if biggest_cubes.blue > MAX_CUBE.blue {
        is_valid = false;
    }

    GameInfo {
        number: game_number,
        cubes: biggest_cubes,
        valid: is_valid,
    }
}

fn calculate_score(games: &Vec<GameInfo>) -> u32 {
    let mut score: u32 = 0;
    for game in games {
        score += game.cubes.red * game.cubes.green * game.cubes.blue;
    }
    score
}

fn main() {
    let file = "game.txt";
    let games: Vec<GameInfo> = fs::read_to_string(file)
        .expect("Unable to read file")
        .lines()
        .map(|line| parse_line(line))
        .collect();

    println!("{}", calculate_score(&games));
}
