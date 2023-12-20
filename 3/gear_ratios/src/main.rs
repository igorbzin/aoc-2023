use std::{fs, i32};

fn main() {
    let file = fs::read_to_string("../input.txt").expect("Text file doesn't exist");
    if file.is_empty() {
        return;
    }

    let vector: Vec<String> = file.lines().map(String::from).collect();
    let line_information_array: Vec<LineInformation> =
        vector.iter().map(|line| process_line(Some(line))).collect();

    let mut sum = 0;
    for (line_number, line_information) in line_information_array.iter().enumerate() {
        let above_line_information = line_information_array.get(line_number.wrapping_sub(1));
        let below_line_information = line_information_array.get(line_number + 1);

        for &(start_idx, end_idx) in &line_information.number_indices {
            let should_add_number =
                check_left_or_right((start_idx, end_idx), &line_information.special_characters)
                    || check_up_or_down(
                        (start_idx, end_idx),
                        above_line_information,
                        below_line_information,
                    );

            if should_add_number {
                if let Some(line) = vector.get(line_number) {
                    match extract_number(line, start_idx as usize, end_idx as usize) {
                        Ok(number) => {
                            println!("Adding number: {}", number);
                            sum += number;
                        }
                        Err(e) => println!("Error occurred: {}", e),
                    }
                }
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

fn check_left_or_right(indices: (i32, i32), special_characters: &Vec<i32>) -> bool {
    if special_characters.contains(&(indices.0 - 1))
        || special_characters.contains(&(indices.1 + 1))
    {
        return true;
    }

    return false;
}

fn check_up_or_down(
    indices: (i32, i32),
    special_characters_above: Option<&LineInformation>,
    special_characters_below: Option<&LineInformation>,
) -> bool {
    if let Some(line_above) = special_characters_above {
        if line_above
            .special_characters
            .iter()
            .any(|character| ((character >= &(indices.0 - 1)) && (character <= &((indices.1) + 1))))
        {
            return true;
        }
    }

    if let Some(line_below) = special_characters_below {
        if line_below
            .special_characters
            .iter()
            .any(|character| (character >= &(indices.0 - 1)) && (character <= &((indices.1) + 1)))
        {
            return true;
        }
    }

    return false;
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
                if !line
                    .chars()
                    .nth(n + 1)
                    .expect("Something went wrong")
                    .is_numeric()
                {
                    number_indices.push((n as i32, n as i32));
                    first_digit_idx = -1;
                }
            } else if character.is_numeric() && first_digit_idx != -1 {
                if n + 1 < line.len()
                    && line
                        .chars()
                        .nth(n + 1)
                        .expect("Something went wrong")
                        .is_numeric()
                {
                    continue;
                }
                number_indices.push((first_digit_idx, n.try_into().unwrap()));
                first_digit_idx = -1;
            } else if !character.is_numeric() && character != '.' {
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
