fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut cards: Vec<Card> = Vec::new();
    let mut sum = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.splitn(2, ':').collect();
        let card_id: usize = parts[0]
            .trim()
            .strip_prefix("Card ")
            .unwrap()
            .parse()
            .unwrap_or(0);

        let numbers: Vec<Vec<usize>> = parts[1]
            .split('|')
            .map(|part| {
                part.trim()
                    .split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect()
            })
            .collect();

        let card = Card {
            id: card_id,
            winning_numbers: numbers[0].clone(),
            my_numbers: numbers[1].clone(),
        };

        cards.push(card);
    }

    cards.iter().for_each(|card| {
        let common_count = card
            .my_numbers
            .iter()
            .filter(|&num| card.winning_numbers.contains(num))
            .count();
        println!("Common Count: {}", common_count);

        if common_count > 0 {
            sum += i32::pow(2, (common_count as u32) - 1);
        }
        println!("Card ID: {}", card.id)
    });

    sum.to_string()
}

struct Card {
    id: usize,
    winning_numbers: Vec<usize>,
    my_numbers: Vec<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, "13");
    }
}
