use std::{sync::mpsc, thread};

pub fn process(input: &str) -> u32 {
    // create two vectors from first half and second half
    // create matches variable
    // iterate over second half checking if first half contains each value
    // add up number of matches
    // return number of matches on card and go next
    let input_lines: Vec<&str> = input.lines().collect();
    let total_cards: u32 = evaluate_cards(&input_lines, (0, input_lines.len()));

    return total_cards;
}

fn evaluate_cards(input: &[&str], range: (usize, usize)) -> u32 {
    let mut total_cards: u32 = 0;
    for i in range.0..range.1 {
        let matches: u32 = find_card_value(input[i]);
        total_cards += 1;

        total_cards += evaluate_cards(input, (&i + 1, &i + 1 + matches as usize));
    }

    return total_cards;
}

fn find_card_value(input: &str) -> u32 {
    let values: &str = input.split(":").last().expect("getting string after :");

    let winning: Vec<u32> = values.split("|").next().expect("getting winning values")
        .split(" ").filter_map(|digit| digit.parse().ok() ).collect();

    let on_card: Vec<u32> = values.split("|").last().expect("getting values on card")
        .split(" ").filter_map(|digit| digit.parse().ok() ).collect();

    let mut matches: u32 = 0;
    for value in &on_card {
        if winning.contains(value) {
            matches += 1;
        }
    }

    return matches;
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str =
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
         Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
         Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
         Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
         Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
         Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_process() {
        assert_eq!(30, process(INPUT));
    }
}
