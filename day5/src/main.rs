use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::take_until,
    error::make_error,
    multi::{self, many0},
    IResult,
};

struct AlmanachMap {
    source: usize,
    destination_offset: usize,
    length: usize,
}

struct Almanach {
    seeds: Vec<usize>,
    seed_to_soil: Vec<AlmanachMap>,
    soil_to_fertilizer: Vec<AlmanachMap>,
    fertilizer_to_water: Vec<AlmanachMap>,
    water_to_light: Vec<AlmanachMap>,
    light_to_temperature: Vec<AlmanachMap>,
    temperature_to_humidity: Vec<AlmanachMap>,
    humidity_to_location: Vec<AlmanachMap>,
}

impl Almanach {
    fn get_location(&self, seed: &usize) -> usize {
        let soil = get_destination(&self.seed_to_soil, seed);
        let fertilizer = get_destination(&self.soil_to_fertilizer, &soil);
        let water = get_destination(&self.fertilizer_to_water, &fertilizer);
        let light = get_destination(&self.water_to_light, &water);
        let temp = get_destination(&self.light_to_temperature, &light);
        let humidity = get_destination(&self.temperature_to_humidity, &temp);
        get_destination(&self.humidity_to_location, &humidity)
    }

    fn get_smallest_location(&self) -> Option<usize> {
        self.seeds.iter().map(|seed| self.get_location(seed)).min()
    }
}

impl AlmanachMap {
    fn get_destination(&self, value: &usize) -> Option<usize> {
        if value >= &self.source && value < &(self.source + self.length) {
            return Some(value + self.destination_offset);
        }
        None
    }
}

fn get_destination(map: &Vec<AlmanachMap>, value: &usize) -> usize {
    let mut result: usize = *value;
    for map in map.iter() {
        if let Some(destination) = map.get_destination(value) {
            result = destination;
            break;
        }
    }
    result
}

fn main() -> Result<()> {
    println!("Hello, world!");

    Ok(())
}

//***********************************************
// Parsing logic
//***********************************************

fn get_number(input: &str) -> IResult<&str, usize> {
    let input = input.trim();
    let (rest, num_str) = take_until(" ")(input)?;
    match num_str.trim().parse() {
        Ok(result) => Ok((rest, result)),
        Err(e) => panic!(),
    }
}

fn get_number_row(row: &str) {
    let row = row.trim();
    let numbers = many0(get_number)(row);
}
