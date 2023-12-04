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

impl Schematic {
    pub fn get_part_number_sum(&self) -> usize {
        let mut sum: usize = 0;
        let mut second_last_line: Option<&SchematicLine> = None;
        let mut last_line: Option<&SchematicLine> = None;
        self.lines.iter().for_each(|line| {
            match (last_line, second_last_line) {
                (None, _) => {}
                (Some(last_line), second_last_line) => {
                    let second_last_line_symbol_positions = match second_last_line {
                        Some(second_last_line) => second_last_line.get_symbol_positions(),
                        None => vec![],
                    };
                    let line_symbol_positions = line.get_symbol_positions();
                    let part_numbers = last_line.get_part_numbers(
                        &second_last_line_symbol_positions,
                        &line_symbol_positions,
                    );

                    println!("{:?}", part_numbers);

                    sum += part_numbers.iter().sum::<usize>();
                }
            }

            second_last_line = last_line;
            last_line = Some(line);
        });

        let second_last_line_symbol_positions = second_last_line.unwrap().get_symbol_positions();

        let part_numbers = last_line
            .unwrap()
            .get_part_numbers(&second_last_line_symbol_positions, &[]);

        println!("{:?}", part_numbers);
        sum += part_numbers.iter().sum::<usize>();

        sum
    }
}

impl SchematicLine {
    pub fn get_symbol_positions(&self) -> Vec<usize> {
        self.items
            .iter()
            .flat_map(|item| {
                if let LineItem::Symbol(symbol) = item {
                    return Some(symbol.index);
                }
                None
            })
            .collect()
    }

    pub fn get_part_numbers(
        &self,
        previous_line_symbol_positions: &[usize],
        next_line_symbol_positions: &[usize],
    ) -> Vec<usize> {
        self.items
            .iter()
            .flat_map(|item| {
                if let LineItem::Number(number) = item {
                    let mut symbol_positions = self.get_symbol_positions().into_iter();
                    let mut previous_line_symbol_positions = previous_line_symbol_positions.iter();
                    let mut next_line_symbol_positions = next_line_symbol_positions.iter();
                    let min = match number.min_index {
                        0 => number.min_index,
                        _ => number.min_index - 1,
                    };
                    if symbol_positions
                        .any(|symbol| symbol >= min && symbol <= (number.max_index + 1))
                    {
                        return Some(number.number);
                    }
                    if previous_line_symbol_positions
                        .any(|symbol| symbol >= &min && symbol <= &(number.max_index + 1))
                    {
                        return Some(number.number);
                    }
                    if next_line_symbol_positions
                        .any(|symbol| symbol >= &min && symbol <= &(number.max_index + 1))
                    {
                        return Some(number.number);
                    }
                }
                None
            })
            .collect()
    }
}

fn main() -> Result<()> {
    let input = Path::new("./input");

    let schematic = get_schematic(input)?;

    println!("Part Sum: {}", schematic.get_part_number_sum());

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
                    min_index = index;
                }
                number_accumulator.push(char);
                if index == line_length - 1 {
                    if let Ok(number) = number_accumulator.parse() {
                        items.push(LineItem::Number(LineItemNumber {
                            min_index,
                            max_index: index - 1,
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
                            max_index: index - 1,
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
