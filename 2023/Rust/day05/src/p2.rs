use std::{collections::HashMap, ops::{RangeInclusive, Range}};

pub fn process(input: &str) -> u32 {
    // Get lowest location number
    // every map is destination range start, source range start, range length
    // range length includes start
    // 50 98 2 means seeds 98-99 coorespond to 50-51
    // Get vec of seed ranges
    // build maps
    // send seed ranges through maps
    // sort vec grab first?
    //
    // TODO: optimizations
    //  Could multithread seed lookup
    //  Could also combine all the maps into one

    println!("Getting seed ranges...");
    let seed_ranges: Vec<Range<u32>> = get_seeds(input);
    println!("Getting maps...");
    let maps: HashMap<&str, Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>> = get_maps(input);

    // This could pretty easily be multithreaded
    let mut location_lowest: u32 = u32::MAX;
    println!("Getting locations...");
    for range in seed_ranges {
        for seed in range {
            let location: u32 = translate_seed(seed, &maps);
            if location < location_lowest {
                location_lowest = location;
            }
        }
    }

    return location_lowest;
}

fn get_seeds(input: &str) -> Vec<Range<u32>> {
    // Seeds are a range
    let numbers: Vec<u32> = input.lines().next().expect("getting first line of input where all seeds are located")
        .split(":")
        .last().expect("getting everything after seeds: label")
        .split_whitespace()
        .map(|integer| integer.parse().expect("parsing seed number into u32"))
        .collect();

    let mut seed_ranges: Vec<Range<u32>> = vec![];
    for i in 0..numbers.len() {
        if i % 2 == 0 {
            // Range starts are going to be on even indices
            let range: Range<u32> = numbers[i]..(numbers[i] + numbers[i + 1]);
            seed_ranges.push(range);
        }
    }

    return seed_ranges;
}

fn get_maps(input: &str) -> HashMap<&str, Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>> {
    let mut maps: HashMap<&str, Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>> = HashMap::new();
    let lines: Vec<&str> = input.lines().collect();

    let mut map_ranges: Vec<(usize, usize)> = vec![];
    let mut map_range: (usize, usize) = (0, 0);
    println!("  Getting map indices...");
    for (index, line) in input.lines().enumerate() {
        if line.contains("map:") {
            map_range = (index, index);
            // Adds first line of map range including label
        }
        if line.trim().is_empty() && map_range != (0, 0) {
            // Checks for an empty line which appears after most maps
            map_range = (map_range.0, index);
            // Adds the last line of the map to the map range
            map_ranges.push(map_range);
            map_range = (0, 0);
            // Resetting value of map_range
        }
        if index == (lines.len() - 1) {
            map_range = (map_range.0, index);
            // Handles a map that ends on the final line of input
            map_ranges.push(map_range);
        }
    }


    println!("  Creating maps...");
    for range in map_ranges {
        let mut ranges_vec: Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> = vec![];
        let mut label: &str = "";

        for line in &lines[range.0..range.1] {
            if line.contains("map") {
                label = line.trim().split_whitespace().next().expect("getting map label");
            }
            else {
                let range_info: Vec<u32> = line.trim().split_whitespace()
                    .map(|number| number.parse().expect("parsing number into u32"))
                    .collect();
                let destination_range_start: u32 = range_info[0];
                let source_range_start: u32 = range_info[1];
                let range_length: u32 = range_info[2];
                // We can safely access these by index because they are
                //  always 3 numbers and in the same order

                println!("      Inserting into {}", label);
                let source_range: RangeInclusive<u32> = source_range_start..=(source_range_start + range_length - 1);
                let destination_range: RangeInclusive<u32> = destination_range_start..=(destination_range_start + range_length - 1);
                ranges_vec.push((source_range, destination_range));
            }
        }
        maps.insert(label, ranges_vec);
    }

    return maps;
}

fn translate_seed(seed: u32, maps: &HashMap<&str, Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>>) -> u32 {

    let soil: u32 = check_map(seed, "seed-to-soil", maps);
    let fertilizer: u32 = check_map(soil, "soil-to-fertilizer", maps);
    let water: u32 = check_map(fertilizer, "fertilizer-to-water", maps);
    let light: u32 = check_map(water, "water-to-light", maps);
    let temperature: u32 = check_map(light, "light-to-temperature", maps);
    let humidity: u32 = check_map(temperature, "temperature-to-humidity", maps);
    let location: u32 = check_map(humidity, "humidity-to-location", maps);

    return location;
}

fn check_map(given_num: u32, ranges_vec: &str, maps: &HashMap<&str, Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>>) -> u32 {
    let mut soil: u32 = given_num;
    for (s_range, d_range) in maps.get(ranges_vec).expect("getting ranges") {
        if s_range.contains(&given_num) {
            let start: u32 = *s_range.start();
            let offset: u32 = given_num - start;
            soil = d_range.start() + offset;
        }
    }

    return soil;

}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str =
        "seeds: 79 14 55 13

         seed-to-soil map:
         50 98 2
         52 50 48

         soil-to-fertilizer map:
         0 15 37
         37 52 2
         39 0 15

         fertilizer-to-water map:
         49 53 8
         0 11 42
         42 0 7
         57 7 4

         water-to-light map:
         88 18 7
         18 25 70

         light-to-temperature map:
         45 77 23
         81 45 19
         68 64 13

         temperature-to-humidity map:
         0 69 1
         1 0 69

         humidity-to-location map:
         60 56 37
         56 93 4";

    #[test]
    fn test_process() {
        assert_eq!(46, process(INPUT));
    }

    #[test]
    fn test_get_seeds() {
        assert_eq!(vec![79..93, 55..68], get_seeds(INPUT));
    }
}
