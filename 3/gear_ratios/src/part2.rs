use std::{fs, i32, usize};

fn main() {
    let file = fs::read_to_string("../input.txt").expect("Text file doesn't exist");
    if file.is_empty() {
        return;
    }

    let lines: Vec<String> = file.lines().map(String::from).collect();
    let line_information_array: Vec<LineInformation> =
        lines.iter().map(|line| process_line(Some(line))).collect();

    let mut sum = 0;
    for (line_number, line_information) in line_information_array.iter().enumerate() {
        let above_line_information = line_information_array.get(line_number.wrapping_sub(1));
        let below_line_information = line_information_array.get(line_number + 1);

        for special_character in &line_information.special_characters {
            let mut adjacent_numbers: Vec<i32> = vec![];

            match above_line_information {
                Some(line_above) => {
                    match check_line(&line_above.number_indices, *special_character) {
                        Ok(found_above) => {
                            found_above.iter().for_each(|(first_index, second_index)| {
                                if let Some(again) =
                                    lines.get(line_number.wrapping_sub(1)).and_then(|test| {
                                        println!("ABOVE LINE {test}");
                                        extract_number(
                                            test,
                                            *first_index as usize,
                                            *second_index as usize,
                                        )
                                        .ok()
                                    })
                                {
                                    adjacent_numbers.push(again);
                                }
                            });
                        }
                        Err(e) => {
                            println!("Error {e}");
                        }
                    }
                }
                None => println!("SUMTHIN HAPPND"),
            }

            //current line
            match check_line(&line_information.number_indices, *special_character) {
                Ok(found_current_line) => {
                    found_current_line
                        .iter()
                        .for_each(|(first_index, second_index)| {
                            if let Some(again) = lines.get(line_number).and_then(|test| {
                                println!("CURRENT LINE {test}");
                                extract_number(test, *first_index as usize, *second_index as usize)
                                    .ok()
                            }) {
                                adjacent_numbers.push(again);
                            }
                        })
                }
                Err(e) => {
                    println!("Error {e}");
                }
            }

            match below_line_information {
                Some(line_below) => {
                    match check_line(&line_below.number_indices, *special_character) {
                        Ok(found_below) => {
                            found_below.iter().for_each(|(first_index, second_index)| {
                                if let Some(again) = lines.get(line_number + 1).and_then(|test| {
                                    println!("BELOW LINE {test}");
                                    extract_number(
                                        test,
                                        *first_index as usize,
                                        *second_index as usize,
                                    )
                                    .ok()
                                }) {
                                    adjacent_numbers.push(again);
                                }
                            })
                        }
                        Err(e) => {
                            println!("Error {e}");
                        }
                    }
                }
                None => println!("SUMTHIN HAPPND"),
            }

            let adjacent = adjacent_numbers
                .iter()
                .map(|number| number.to_string())
                .collect::<Vec<String>>()
                .join(", ");

            println!("AFTER EXAMINING ALL ADJACENT NUMBERS {adjacent}");

            let current_line = lines.get(line_number);
            if adjacent_numbers.len() == 2 && current_line.is_some() {
                sum += adjacent_numbers[0] * adjacent_numbers[1];
            }
        }
    }

    println!("Sum of all is: {}", sum);
}

fn extract_number(s: &str, start_idx: usize, end_idx: usize) -> Result<i32, String> {
    if end_idx < start_idx || end_idx >= s.len() {
        return Err("Invalid indices".to_string());
    }

    let substring = &s[start_idx..=end_idx];
    println!("Attempting to parse substring: '{}'", substring); // Debugging line

    substring
        .parse::<i32>()
        .map_err(|e| format!("Failed to parse '{}': {}", substring, e))
}

fn check_line(indices: &Vec<(i32, i32)>, character_idx: i32) -> Result<Vec<(i32, i32)>, String> {
    let mut adjacent_numbers: Vec<(i32, i32)> = vec![];

    let number_indices_str = indices
        .iter()
        .map(|(start, end)| format!("({}, {})", start, end))
        .collect::<Vec<String>>()
        .join(", ");

    println!("Examining special character with idx {character_idx} alongside indices {number_indices_str}");

    for &(first_idx, last_idx) in indices.iter() {
        if (character_idx >= first_idx - 1) && (character_idx <= last_idx + 1) {
            adjacent_numbers.push((first_idx, last_idx));
        }
    }

    if adjacent_numbers.len() > 2 {
        Err("The character already has another adjacent number".to_string())
    } else {
        Ok(adjacent_numbers)
    }
}

fn process_line(line_to_examine: Option<&String>) -> LineInformation {
    let mut number_indices: Vec<(i32, i32)> = Vec::new();
    let mut special_characters: Vec<i32> = Vec::new();

    if let Some(line) = line_to_examine {
        let mut first_digit_idx: i32 = -1;
        for n in 0..line.len() {
            let character = line.chars().nth(n).expect("Not a valid character index");
            if character.is_numeric() && first_digit_idx == -1 {
                first_digit_idx = n as i32;
                if !line.chars().nth(n + 1).unwrap_or('.').is_numeric() {
                    number_indices.push((n as i32, n as i32));
                    first_digit_idx = -1;
                }
            } else if character.is_numeric() && first_digit_idx != -1 {
                if n + 1 < line.len() && line.chars().nth(n + 1).unwrap_or('.').is_numeric() {
                    continue;
                }
                number_indices.push((first_digit_idx, n.try_into().unwrap()));
                first_digit_idx = -1;
            } else if character == '*' {
                special_characters.push(n as i32);
            }
        }
    }
    return LineInformation::new(number_indices, special_characters);
}

struct LineInformation {
    number_indices: Vec<(i32, i32)>,
    special_characters: Vec<i32>,
}

impl LineInformation {
    // Constructor for MyClass with parameters
    fn new(indices: Vec<(i32, i32)>, integers: Vec<i32>) -> Self {
        LineInformation {
            number_indices: indices,
            special_characters: integers,
        }
    }
}
