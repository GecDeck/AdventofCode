pub fn process(input: &str) -> i64 {
    // get differences until we hit a vector of all zeroes
    // sum last differences
    // sum that sum for all lines

    let mut sum: i64 = 0;
    for line in input.lines() {
        let nums: Vec<i64> = get_nums(line);
        let last_num: i64 = get_last(&nums);
        sum += last_num;
    }
    return sum;
}

fn get_nums(input: &str) -> Vec<i64> {
    let mut nums: Vec<i64> = input.split_whitespace()
        .map(|num| num.parse().expect("parsing num into i64"))
        .collect();
    nums.reverse();
    nums.push(0);
    return nums;
}

fn get_last(nums: &[i64]) -> i64 {
    let last: i64 = get_diffs(nums)[0];
    return -last;
}

fn get_diffs(nums: &[i64]) -> Vec<i64> {
    let mut diffs: Vec<i64> = vec![];
    for i in 1..nums.len() {
        let diff: i64 = nums[i - 1] - nums[i];
        diffs.push(diff);
    }
    if diffs.len() != 1 {
        diffs = get_diffs(&diffs);
    }

    return diffs;
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str =
        "0 3 6 9 12 15
         1 3 6 10 15 21
         10 13 16 21 30 45";

    #[test]
    fn test_process() {
        assert_eq!(2, process(INPUT));
    }

    #[test]
    fn test_get_nums() {
        assert_eq!(vec![15, 12, 9, 6, 3, 0, 0], get_nums("0 3 6 9 12 15"));
    }

    #[test]
    fn test_get_last() {
        assert_eq!(-3, get_last(&get_nums("0 3 6 9 12 15")));
    }

    #[test]
    fn test_get_diffs() {
        assert_eq!(vec![3], get_diffs(&get_nums("0 3 6 9 12 15")));
    }
}
