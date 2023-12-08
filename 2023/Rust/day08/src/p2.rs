use std::collections::HashMap;
use itertools::Itertools;

const START_CHAR: char = 'A';
const END_CHAR: char = 'Z';
const RIGHT: char = 'R';
const LEFT: char = 'L';

pub fn process(input: &str) -> u64 {
    let directions: &str = get_directions(input);
    let (map, starts): (HashMap<&str, Path>, Vec<&str>) = get_map(input);
    let steps: u64 = navigate_map(directions, map, starts);
    return steps;
}

fn get_directions(input: &str) -> &str {
    return input
        .lines().next().expect("getting first line of input with directions")
        .trim();
}

struct Path {
    left: String,
    right: String,
}
impl Path {
    fn new(input: &str) -> Self {
        // receives (BBB, CCC)
        let input: String = input
            .replace('(', "")
            .replace(')', "")
            .replace(',', "");
        // BBB CCC
        let (left, right): (&str, &str) = input.split_whitespace()
                             .collect_tuple().expect("collecting directions into tuple");
        Self {
            left: left.to_string(),
            right: right.to_string(),
        }
    }
}

fn get_map(input: &str) -> (HashMap<&str, Path>, Vec<&str>) {
    let mut map: HashMap<&str, Path> = HashMap::new();
    let mut starts: Vec<&str> = vec![];

    for line in input.lines() {
        if line.contains('=') {
            let location: &str = line
                .trim()
                .split('=')
                .next().expect("getting location portion of line")
                .trim();

            if location.ends_with(START_CHAR) {
                starts.push(location);
            }

            let directions: &str = line
                .trim()
                .split('=')
                .last().expect("getting directions portion of line");
            let path: Path = Path::new(directions);
            map.insert(location, path);
        }
    }

    return (map, starts);
}

fn navigate_map(directions: &str, map: HashMap<&str, Path>, starts: Vec<&str>) -> u64 {
    let mut steps_vec: Vec<u64> = vec![];

    for start in starts {
        let mut current_location: &str = start;
        let mut steps: u64 = 0;
        let mut at_end: bool = false;

        while !at_end {
            for char in directions.chars() {
                let paths: &Path = map.get(current_location)
                    .expect("getting directions from current location");
                match char {
                    LEFT => current_location = &paths.left,
                    RIGHT => current_location = &paths.right,
                    _ => panic!("unhandled char found"),
                }
                steps += 1;
                if current_location.ends_with(END_CHAR) {
                    steps_vec.push(steps);
                    at_end = true;
                    break;
                    // Skips to walking map from next start location
                }
            }
        }
    }

    let lcm: u64 = lcm_of_vec(&steps_vec);

    return lcm;
}

fn lcm_of_vec(vec: &[u64]) -> u64 {
    let mut lcm: u64 = 1;

    for x in vec {
        lcm = num::integer::lcm(*x, lcm);
    }

    return lcm;
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str =
        "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)";

    #[test]
    fn test_process() {
        assert_eq!(6, process(INPUT));
    }

    #[test]
    fn test_directions() {
        assert_eq!("LR", get_directions(INPUT));
    }
}
