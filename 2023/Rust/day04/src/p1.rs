pub fn process(input: &str) -> u32 {
    // create two vectors from first half and second half
    // create points variable
    // iterate over second half checking if first half contains each value
    // add 1 to points for first match, multiply sum by 2 for rest of matches
    // return point value of card and go next
    let mut total_points: u32 = 0;
    for card in input.lines() {
        let points: u32 = find_card_value(card);
        total_points += points;
    }

    return total_points;
}

fn find_card_value(input: &str) -> u32 {
    let values: &str = input.split(":").last().expect("getting string after :");

    let winning: Vec<u32> = values.split("|").next().expect("getting winning values")
        .split(" ").filter_map(|digit| digit.parse().ok() ).collect();

    let on_card: Vec<u32> = values.split("|").last().expect("getting values on card")
        .split(" ").filter_map(|digit| digit.parse().ok() ).collect();

    let mut points: u32 = 0;
    for value in &on_card {
        if winning.contains(value) {
            match points {
                0 => points = 1,
                _ => points *= 2,
            }
        }
    }

    return points;
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
        assert_eq!(13, process(INPUT));
    }
}
