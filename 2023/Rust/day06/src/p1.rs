pub fn process(input: &str) -> u32 {
    // create vector of races
    // for each race find number of winning variants
    // multiply each races number of winning variants by each other
    let races: Vec<Race> = get_race_info(input);

    let mut product: u32 = 0;
    for race in races {
        let variants: u32 = get_variants(race);
        if variants != 0 {
            match product {
                0 => product = variants,
                _ => product *= variants,
            }
        }
    }

    return product;
}

#[derive(Debug, PartialEq)]
struct Race {
    time: u32,
    record: u32,
}
impl Race {
    fn new(time: u32, record: u32) -> Self{
        Self {
            time,
            record,
        }
    }
}

fn get_race_info(input: &str) -> Vec<Race> {
    let times: Vec<u32> = input.lines().next().expect("getting line with times on it")
        .split(":").last().expect("getting everything after tag")
        .split_whitespace().map(|num| num.parse().expect("parsing time into u32"))
        .collect();
    let records: Vec<u32> = input.lines().last().expect("getting line with records on it")
        .split(":").last().expect("getting everything after tag")
        .split_whitespace().map(|num| num.parse().expect("parsing record into u32"))
        .collect();

    let mut races: Vec<Race> = vec![];
    for i in 0..times.len() {
        let time: u32 = times[i];
        let record: u32 = records[i];

        races.push(Race::new(time, record));
    }

    return races;
}

fn get_variants(race: Race) -> u32 {
    // Every second you charge the boat is 1 m/s increase in speed
    let mut successes: u32 = 0;

    // If x is number of seconds charged, and y is speed
    // Roots of quadratic equation will be where x = 0 and where x = time
    // It also always passes through the y axis at 0, 0
    for charge_time in 0..=race.time {
        let time_left = race.time - charge_time;
        if charge_time * time_left > race.record {
            successes += 1
        }
    }

    return successes;
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str =
        "Time:      7  15   30
         Distance:  9  40  200";

    #[test]
    fn test_process() {
        assert_eq!(288, process(INPUT));
    }

    #[test]
    fn test_get_race_info() {
        let races: Vec<Race> = vec![
            Race::new(7, 9),
            Race::new(15, 40),
            Race::new(30, 200),
        ];
        assert_eq!(races, get_race_info(INPUT))
    }

    #[test]
    fn test_get_variants() {
        let race: Race = Race::new(7, 9);
        assert_eq!(4, get_variants(race));
    }
}
