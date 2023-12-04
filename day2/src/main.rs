use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::{anyhow, Result};
use nom::bytes::complete::{tag, take_until};

#[derive(Debug, PartialEq)]
struct Game {
    id: usize,
    draws: Vec<Content>,
}

#[derive(Debug, PartialEq)]
struct Content {
    blue: usize,
    red: usize,
    green: usize,
}

impl Game {
    pub fn is_possible(&self, bag_content: &Content) -> bool {
        self.draws.iter().all(|draw| {
            draw.blue <= bag_content.blue
                && draw.red <= bag_content.red
                && draw.green <= bag_content.green
        })
    }

    pub fn get_minimal_bag_content(&self) -> Content {
        let mut max_blue = 0;
        let mut max_red = 0;
        let mut max_green = 0;

        self.draws.iter().for_each(|draw| {
            if draw.blue > max_blue {
                max_blue = draw.blue;
            }
            if draw.red > max_red {
                max_red = draw.red;
            }
            if draw.green > max_green {
                max_green = draw.green;
            }
        });

        Content {
            blue: max_blue,
            red: max_red,
            green: max_green,
        }
    }
}

impl Content {
    fn power(&self) -> usize {
        self.blue * self.red * self.green
    }
}

fn main() -> Result<()> {
    let input = Path::new("./input");
    let check_for_contents = Content {
        red: 12,
        green: 13,
        blue: 14,
    };

    let games = get_games(input)?;

    let sum = get_games_sum(&games, &check_for_contents);

    println!("Games sum: {}", sum);

    let power_sum = get_games_power_sum(&games);

    println!("Games power sum: {}", power_sum);

    Ok(())
}

fn get_games_power_sum(games: &[Game]) -> usize {
    let mut sum: usize = 0;
    games.iter().for_each(|game| {
        sum += game.get_minimal_bag_content().power();
    });
    sum
}

fn get_games_sum(games: &[Game], content: &Content) -> usize {
    let mut sum: usize = 0;
    games.iter().for_each(|game| {
        if game.is_possible(content) {
            sum += game.id;
        }
    });
    sum
}

fn get_games(path: &Path) -> Result<Vec<Game>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let games = reader
        .lines()
        .flatten()
        .flat_map(|line| get_game(&line))
        .collect();

    Ok(games)
}

//*****************************
// Parsing the game
// I'm not very familiar with nom yet. I think, that there is a more elegant (say: less imperative) solution to parse using nom
//*****************************

fn get_game(input: &str) -> Result<Game> {
    let (input, _) = map_nom_err(tag("Game ")(input))?;
    let (input, game_id_str) = map_nom_err(take_until(":")(input))?;
    let game_id = get_number(game_id_str)?;
    let (input, _) = map_nom_err(tag(":")(input))?;

    let draws = get_draws(input)?;

    Ok(Game { id: game_id, draws })
}

fn get_draws(input: &str) -> Result<Vec<Content>> {
    let draw_inputs = input.split(';');

    Ok(draw_inputs.flat_map(get_draw).collect())
}

fn get_draw(input: &str) -> Result<Content> {
    let mut result = Content {
        blue: 0,
        red: 0,
        green: 0,
    };
    let color_inputs = input.split(',');

    for color_input in color_inputs {
        let color_input = color_input.trim();
        let (color_input, count_str) = map_nom_err(take_until(" ")(color_input))?;
        let count = get_number(count_str)?;
        let color_input = color_input.trim();

        match color_input {
            "red" => result.red = count,
            "green" => result.green = count,
            "blue" => result.blue = count,
            _ => eprintln!("{} is no color", color_input),
        }
    }
    Ok(result)
}

fn get_number(input: &str) -> Result<usize> {
    Ok(input.trim().parse()?)
}

fn map_nom_err<'a>(
    nom_result: nom::IResult<&'a str, &'a str, nom::error::Error<&str>>,
) -> Result<(&'a str, &'a str)> {
    match nom_result {
        Ok(tuple) => Ok(tuple),
        Err(_) => Err(anyhow!("Error parsing")),
    }
}
