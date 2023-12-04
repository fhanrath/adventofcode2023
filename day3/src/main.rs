use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::Result;

#[derive(Debug, PartialEq)]
struct Schematic {
    lines: Vec<SchematicLine>,
}

#[derive(Debug, PartialEq)]
struct SchematicLine {
    items: Vec<LineItem>,
}

#[derive(Debug, PartialEq)]
enum LineItem {
    Number(LineItemNumber),
    Symbol(LineItemSymbol),
}

#[derive(Debug, PartialEq)]
struct LineItemNumber {
    min_index: usize,
    max_index: usize,
    number: usize,
}

#[derive(Debug, PartialEq)]
struct LineItemSymbol {
    pub index: usize,
    pub is_star: bool,
}

#[derive(Debug, PartialEq)]
struct CompiledSchematic {
    items: Vec<CompiledItem>,
}

#[derive(Debug, PartialEq)]
enum CompiledItem {
    PartNumber(PartNumber),
    Gear(Gear),
}

#[derive(Debug, PartialEq)]
struct PartNumber {
    min_index: usize,
    max_index: usize,
    number: usize,
}

#[derive(Debug, PartialEq)]
struct Gear {
    pub index: usize,
    pub gear_ratio: usize,
}

impl CompiledSchematic {
    pub fn get_part_sum(&self) -> usize {
        self.items
            .iter()
            .flat_map(|item| match item {
                CompiledItem::PartNumber(number) => Some(number.number),
                CompiledItem::Gear(_) => None,
            })
            .sum::<usize>()
    }

    pub fn get_gear_ratio_sum(&self) -> usize {
        self.items
            .iter()
            .flat_map(|item| match item {
                CompiledItem::PartNumber(_) => None,
                CompiledItem::Gear(gear) => Some(gear.gear_ratio),
            })
            .sum::<usize>()
    }
}

impl Schematic {
    pub fn compile(&mut self) -> CompiledSchematic {
        let mut second_last_line: Option<&SchematicLine> = None;
        let mut last_line: Option<&SchematicLine> = None;
        let mut numbers_gears: Vec<Vec<CompiledItem>> = vec![];
        self.lines.iter().for_each(|line| {
            match (last_line, second_last_line) {
                (None, _) => {}
                (Some(last_line), second_last_line) => {
                    let second_last_line_items_vec = vec![];
                    let mut second_last_line_items = &second_last_line_items_vec;
                    if let Some(second_last_line) = second_last_line {
                        second_last_line_items = &second_last_line.items;
                    };
                    let current_line_items = &last_line.items;
                    let next_line_items = &line.items;

                    numbers_gears.push(get_part_numbers_and_gears(
                        second_last_line_items,
                        current_line_items,
                        next_line_items,
                    ));
                }
            }

            second_last_line = last_line;
            last_line = Some(line);
        });

        let second_last_line_items = &second_last_line.unwrap().items;
        let current_line_items = &last_line.unwrap().items;
        let empty: Vec<LineItem> = vec![];

        numbers_gears.push(get_part_numbers_and_gears(
            second_last_line_items,
            current_line_items,
            &empty,
        ));

        let numbers_gears: Vec<CompiledItem> = numbers_gears.into_iter().flatten().collect();

        CompiledSchematic {
            items: numbers_gears,
        }
    }
}

fn get_part_numbers_and_gears(
    previous_items: &[LineItem],
    current_items: &[LineItem],
    next_items: &[LineItem],
) -> Vec<CompiledItem> {
    let mut return_value: Vec<CompiledItem> = vec![];

    let (previous_numbers, previous_symbols) = split_numbers_symbols(previous_items);
    let (current_numbers, current_symbols) = split_numbers_symbols(current_items);
    let (next_numbers, next_symbols) = split_numbers_symbols(next_items);

    current_items.iter().for_each(|item| match item {
        LineItem::Number(number) => {
            let mut current_symbols = current_symbols.iter();
            let mut previous_symbols = previous_symbols.iter();
            let mut next_symbols = next_symbols.iter();

            if current_symbols.any(|symbol| is_part(number, symbol)) {
                return_value.push(CompiledItem::PartNumber(PartNumber {
                    min_index: number.min_index,
                    max_index: number.max_index,
                    number: number.number,
                }));
            }
            if previous_symbols.any(|symbol| is_part(number, symbol)) {
                return_value.push(CompiledItem::PartNumber(PartNumber {
                    min_index: number.min_index,
                    max_index: number.max_index,
                    number: number.number,
                }));
            }
            if next_symbols.any(|symbol| is_part(number, symbol)) {
                return_value.push(CompiledItem::PartNumber(PartNumber {
                    min_index: number.min_index,
                    max_index: number.max_index,
                    number: number.number,
                }));
            }
        }
        LineItem::Symbol(symbol) => {
            if symbol.is_star {
                let current_numbers: Vec<usize> = current_numbers
                    .iter()
                    .filter(|number| is_part(number, symbol))
                    .map(|number| number.number)
                    .collect();
                let previous_numbers: Vec<usize> = previous_numbers
                    .iter()
                    .filter(|number| is_part(number, symbol))
                    .map(|number| number.number)
                    .collect();
                let next_numbers: Vec<usize> = next_numbers
                    .iter()
                    .filter(|number| is_part(number, symbol))
                    .map(|number| number.number)
                    .collect();

                if current_numbers.len() + previous_numbers.len() + next_numbers.len() == 2 {
                    let gear_ratio = current_numbers
                        .into_iter()
                        .reduce(|acc, item| acc * item)
                        .unwrap_or(1)
                        * previous_numbers
                            .into_iter()
                            .reduce(|acc, item| acc * item)
                            .unwrap_or(1)
                        * next_numbers
                            .into_iter()
                            .reduce(|acc, item| acc * item)
                            .unwrap_or(1);

                    return_value.push(CompiledItem::Gear(Gear {
                        index: symbol.index,
                        gear_ratio,
                    }));
                }
            }
        }
    });

    return_value
}

fn split_numbers_symbols(items: &[LineItem]) -> (Vec<&LineItemNumber>, Vec<&LineItemSymbol>) {
    let mut numbers: Vec<&LineItemNumber> = vec![];
    let mut symbols: Vec<&LineItemSymbol> = vec![];
    items.iter().for_each(|item| match item {
        LineItem::Number(number) => numbers.push(number),
        LineItem::Symbol(symbol) => symbols.push(symbol),
    });

    (numbers, symbols)
}

fn is_part(number: &LineItemNumber, symbol: &LineItemSymbol) -> bool {
    symbol.index >= number.min_index && symbol.index <= number.max_index
}

fn main() -> Result<()> {
    let input = Path::new("./input");

    let mut schematic = get_schematic(input)?;

    let schematic = schematic.compile();

    println!("Part Sum: {}", schematic.get_part_sum());
    println!("Gear Score Sum: {}", schematic.get_gear_ratio_sum());

    Ok(())
}

fn get_schematic(path: &Path) -> Result<Schematic> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let lines = reader
        .lines()
        .flatten()
        .flat_map(|line| parse_schematic_line(&line))
        .collect();

    Ok(Schematic { lines })
}

//*****************************
// Parsing the line
//*****************************
fn parse_schematic_line(line: &str) -> Result<SchematicLine> {
    let mut number_accumulator = String::new();
    let mut min_index = 0;
    let mut items: Vec<LineItem> = Vec::new();
    let line_length = line.len();
    line.chars()
        .enumerate()
        .for_each(|(index, char)| match char {
            '0'..='9' => {
                if number_accumulator.is_empty() {
                    min_index = match index {
                        0 => index,
                        _ => index - 1,
                    };
                }
                number_accumulator.push(char);
                if index == line_length - 1 {
                    if let Ok(number) = number_accumulator.parse() {
                        items.push(LineItem::Number(LineItemNumber {
                            min_index,
                            max_index: index,
                            number,
                        }));
                    };
                }
            }
            _ => {
                if !number_accumulator.is_empty() {
                    if let Ok(number) = number_accumulator.parse() {
                        items.push(LineItem::Number(LineItemNumber {
                            min_index,
                            max_index: index,
                            number,
                        }));
                    };

                    number_accumulator = String::new();
                }
                if char != '.' {
                    items.push(LineItem::Symbol(LineItemSymbol {
                        index,
                        is_star: char == '*',
                    }));
                }
            }
        });
    Ok(SchematicLine { items })
}
