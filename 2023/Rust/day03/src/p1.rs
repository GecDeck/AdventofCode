use std::collections::HashMap;

pub fn process(input: &str) -> u64 {
    // A valid part number is one where it touches a symbol even diagonally
    // find symbol indices
    // check line, line before and line after at indices +- 1 of symbol
    // if you find a number walk forward and backward until you hit a non numeric value
    // get numbers and add to sum
    let symbols: Vec<(usize, usize)> = find_symbols(input);
    let positions: Vec<(usize, usize)> = find_number_positions(input, &symbols);
    let numbers: Vec<u64> = find_numbers(input, &positions);

    let sum: u64 = numbers.iter().map(|x| x).sum();
    return sum;
}

fn find_symbols(input: &str) -> Vec<(usize, usize)> {
    // Given a string return a vec of all symbols where (index in line, line number)

    let mut symbols: Vec<(usize, usize)> = vec![];
    for (line_num, line) in input.lines().enumerate() {
        for (index, char) in line.trim().chars().enumerate() {
            if !char.is_digit(10) && char != '.' {
                symbols.push((index, line_num));
            }
        }
    }

    return symbols;
}

fn find_number_positions(input: &str, symbols: &[(usize, usize)]) -> Vec<(usize, usize)> {
    // Given a string and list of symbols return a vector of numbers bordering those symbols

    let mut adjacent_number_indices: Vec<(usize, usize)> = vec![];
    // index in line, line number

    let lines: Vec<&str> = input.lines().collect();
    for symbol in symbols {

        let lower_range = if symbol.1 == 0 { 0 } else { symbol.1 - 1 };
        let upper_range = if symbol.1 == lines.len() - 1 { lines.len() - 1 } else { symbol.1 + 1 };

        for (relative_line, line) in lines[lower_range..=upper_range].iter().enumerate() {
            let chars: Vec<char> = line.trim().chars().collect();

            let lower_range = if symbol.0 == 0 { 0 } else { symbol.0 - 1 };
            let upper_range = if symbol.0 == chars.len() - 1 { chars.len() - 1 } else { symbol.0 + 1 };

            for (relative_index, char) in chars[lower_range..=upper_range].iter().enumerate() {
                if char.is_digit(10) {
                    let line_number: usize = match relative_line {
                        0 => symbol.1 - 1,
                        1 => if symbol.1 == 0 { symbol.1 + 1 } else { symbol.1 },
                        2 => symbol.1 + 1,
                        _ => panic!(),
                    };

                    let index: usize = match relative_index {
                        0 => if symbol.0 == 0 { 0 } else { symbol.0 - 1 },
                        1 => symbol.0,
                        2 => if symbol.0 == chars.len() - 1 { chars.len() - 1 } else { symbol.0 + 1 },
                        _ => panic!(),
                    };

                    let adjacent_number_position: (usize, usize) = (index, line_number);
                    adjacent_number_indices.push(adjacent_number_position);
                }
            }
        }
    }

    return adjacent_number_indices;
}

fn find_numbers(input: &str, number_positions: &[(usize, usize)]) -> Vec<u64> {
    // walk through line gathering numbers and the index of each digit
    // check each number to check if any of its digits are found in number_positions

    let mut number_map: HashMap<(usize, usize), u64> = HashMap::new();
    for (line_num, line) in input.lines().enumerate() {
        let mut number: Vec<char> = vec![];
        let mut digit_positions: Vec<(usize, usize)> = vec![];
        for (index, char) in line.trim().chars().enumerate() {
            if char.is_digit(10) {
                number.push(char);
                digit_positions.push((index, line_num));
            }

            if !char.is_digit(10) || index == line.trim().len() - 1 {
                for digit_position in &digit_positions {
                    if number_positions.contains(digit_position) && number.len() != 0 {
                        let number_string: String = number.iter().collect();
                        let number_u64: u64 = number_string.parse().expect("parsing string into u64");

                        match number_map.get(digit_position) {
                            Some(_) => { },
                            None => { number_map.insert(*digit_position, number_u64); },
                        }
                        number.clear();
                    }
                }
                number.clear();
                digit_positions.clear();
            }
        }
    }

    let mut numbers: Vec<u64> = vec![];
    for (_, v) in number_map {
        numbers.push(v);
    }
    return numbers;
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str =
        "467..114..
         ...*......
         ..35..633.
         ......#...
         617*......
         .....+.58.
         ..592...12
         ......755.
         ...$.*....
         .664.598..";

    const INPUT2: &str =
        "12.......*..
         +.........34
         .......-12..
         ..78........
         ..*....60...
         78.........9
         .5.....23..$
         8...90*12...
         ............
         2.2......12.
         .*.........*
         1.1..503+.56";

    const INPUT3: &str =
        "100
         200";

    #[test]
    fn test_process() {
        assert_eq!(4361, process(INPUT));
        assert_eq!(925, process(INPUT2));
        assert_eq!(0, process(INPUT3));
    }

    #[test]
    fn test_find_symbols() {
        let symbols: Vec<(usize, usize)> = vec![
            (3, 1), (6, 3), (3, 4), (5, 5), (3, 8), (5, 8)
        ];
        assert_eq!(symbols, find_symbols(INPUT));

        let symbols2: Vec<(usize, usize)> = vec![
            (9, 0), (0, 1), (7, 2), (2, 4), (11, 6), (6, 7), (1, 10), (11, 10), (8, 11)
        ];
        assert_eq!(symbols2, find_symbols(INPUT2));
    }

    #[test]
    fn test_find_number_positions() {
        let symbols: Vec<(usize, usize)> = find_symbols(INPUT);
        let positions: Vec<(usize, usize)> = vec![
            (2, 0), (2, 2), (3, 2), (6, 2), (7, 2), (2, 4), (4, 6), (2, 9), (3, 9), (6, 7), (5, 9), (6, 9)
        ];
        assert_eq!(positions, find_number_positions(INPUT, &symbols));
    }

    #[test]
    fn test_find_numbers() {
        let symbols: Vec<(usize, usize)> = find_symbols(INPUT);
        let positions: Vec<(usize, usize)> = find_number_positions(INPUT, &symbols);
        let mut numbers: Vec<u64> = vec![
            467, 35, 633, 617, 592, 755, 664, 598
        ];
        numbers.sort();

        let mut result_numbers: Vec<u64> = find_numbers(INPUT, &positions);
        result_numbers.sort();

        assert_eq!(numbers, result_numbers);

        let symbols2: Vec<(usize, usize)> = find_symbols(INPUT2);
        let positions2: Vec<(usize, usize)> = find_number_positions(INPUT2, &symbols2);
        let mut numbers2: Vec<u64> = vec![
            12, 34, 12, 78, 78, 9, 23, 90, 12, 2, 2, 12, 1, 1, 503, 56
        ];
        numbers2.sort();

        let mut result_numbers2: Vec<u64> = find_numbers(INPUT2, &positions2);
        result_numbers2.sort();

        assert_eq!(numbers2, result_numbers2);
    }
}
