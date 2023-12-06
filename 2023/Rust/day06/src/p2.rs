pub fn process(input: &str) -> u64 {
    // Get race info
    // Find number of winning variants
    let race: Race = get_race_info(input);

    let variants: u64 = get_variants(race);

    return variants;
}

#[derive(Debug, PartialEq)]
struct Race {
    time: u64,
    record: u64,
}
impl Race {
    fn new(time: u64, record: u64) -> Self{
        Self {
            time,
            record,
        }
    }
}

fn get_race_info(input: &str) -> Race {
    let time: u64 = input.lines().next().expect("getting line with times on it")
        .split(":").last().expect("getting everything after tag")
        .replace(" ", "").parse().expect("parsing time string into u64");

    let record: u64 = input.lines().last().expect("getting line with records on it")
        .split(":").last().expect("getting everything after tag")
        .replace(" ", "").parse().expect("parsing records string into u64");

    return Race::new(time, record);
}

fn get_variants(race: Race) -> u64 {
    // Every second you charge the boat is 1 m/s increase in speed
    let mut successes: u64 = race.time + 1;
    // Maximum possible variants is the time limit + 1

    for charge_time in 0..=race.time {
        let time_left = race.time - charge_time;
        if charge_time * time_left < race.record {
            successes -= 2;
            // Every failure has an equivalent failure on the opposite side of the parabella
        }
        else if charge_time * time_left > race.record {
            return successes;
            // We've already handled remove all failures from both sides
            //  so if we get a success then all variants left are successes so we can return now
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
        assert_eq!(71503, process(INPUT));
    }

    #[test]
    fn test_get_race_info() {
        let race: Race = Race::new(71530, 940200);
        assert_eq!(race, get_race_info(INPUT))
    }
}
