use std::collections::HashMap;

const CARD_PRIO: &[char; 13] = &['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];
const WILDCARD: char = 'J';

pub fn process(input: &str) -> u32 {
    let mut hands: Vec<Hand> = get_hands(input);
    hands.sort_by(|a, b| {
                  let a_num: u32 = a.get_unique_to(b);
                  let b_num: u32 = b.get_unique_to(a);
                  a_num.partial_cmp(&b_num).unwrap()
    });

    let mut total_winnings: u32 = 0;
    for i in 0..hands.len() {
        total_winnings += hands[i].bet * (i as u32 + 1);
    }


    // assert!(total_winnings > 251358097);
    return total_winnings;
}

enum HandTypes {
    Five = 6,
    Four = 5,
    Full = 4,
    Three = 3,
    TwoPair = 2,
    Pair = 1,
    High = 0,
}

#[derive(Debug, PartialEq)]
struct Hand {
    bet: u32,
    prio: Vec<u32>,
}
impl Hand {
    fn new(input: &str) -> Self {
        let hand: Vec<&str> = input.split_whitespace().collect();
        let bet: u32 = hand[1].parse().expect("parsing bet into u32");
        let cards: Vec<char> = hand[0].chars().collect();

        // Get card priorities first
        let mut prio: Vec<u32> = vec![];
        for card in &cards {
            for i in 0..CARD_PRIO.len() {
                if card == &CARD_PRIO[i] {
                    prio.push(i as u32);
                }
            }
        }

        // Get hand priority
        let mut freq_map: HashMap<char, u32> = HashMap::new();
        for card in &cards {
            match freq_map.get_mut(&card) {
                Some(result) => *result += 1,
                None => { match freq_map.insert(*card, 1) {
                    Some(ans) => println!("{:?}", ans),
                    None => {},
                }},
            }
        }

        let num_wildcard: u32 = match freq_map.get(&WILDCARD) {
            Some(result) => *result,
            None => 0,
        };
        let mut most_freq: (char, u32) = ('J', 0);
        if cards.contains(&WILDCARD) {
            for (k, v) in &freq_map {
                if *v > most_freq.1 && *k != WILDCARD {
                    most_freq = (*k, *v)
                }
            }
            if most_freq != ('J', 0) {
                match freq_map.get_mut(&most_freq.0) {
                    Some(result) => *result += num_wildcard,
                    None => {},
                }
                match freq_map.get_mut(&WILDCARD) {
                    Some(result) => *result = 0,
                    None => {},
                }
            }
        }

        let mut freqs: Vec<u32> = freq_map.iter().map(|(_, v)| *v).collect();
        freqs.sort();

        let mut freq_of_freqs_map: HashMap<u32, u32> = HashMap::new();
        for freq in &freqs {
            match freq_of_freqs_map.get_mut(&freq) {
                Some(result) => *result += 1,
                None => { match freq_of_freqs_map.insert(*freq, 1) {
                    Some(ans) => println!("{:?}", ans),
                    None => {},
                }},
            }
        }
        let mut freq_of_freqs: Vec<(u32, u32)> = freq_of_freqs_map.iter().map(|(k, v)| (*k, *v)).collect();
        freq_of_freqs.sort_by(|a, b| a.0.partial_cmp(&b.0).expect("sorting frequencies"));

        let most_freq: u32 = freq_of_freqs.last().expect("getting most frequent").0;
        match most_freq {
            5 => prio.insert(0, HandTypes::Five as u32),
            4 => prio.insert(0, HandTypes::Four as u32),
            3 => {
                match freq_of_freqs_map.get(&2) {
                    Some(_) => prio.insert(0, HandTypes::Full as u32),
                    None => prio.insert(0, HandTypes::Three as u32),
                }
            },
            2 => {
                match freq_of_freqs_map.get(&2).expect("getting number of pairs") {
                    2.. => prio.insert(0, HandTypes::TwoPair as u32),
                    1 => prio.insert(0, HandTypes::Pair as u32),
                    0 => panic!("Map contained no pairs even though it should")
                }
            },
            1 => prio.insert(0, HandTypes::High as u32),
            _ => panic!("Unhandled exception")
        }

        Self {
            bet,
            prio,
        }
    }

    fn get_unique_to(&self, other_hand: &Hand) -> u32 {
        for i in 0..self.prio.len() {
            if self.prio[i] != other_hand.prio[i] {
                return self.prio[i];
            }
        }
        // Checks each number to find the first one that they differ on

        return self.prio[0];
        // If both hands are identical return just the hand priority
    }
}

fn get_hands(input: &str) -> Vec<Hand> {
    let mut hands: Vec<Hand> = vec![];
    for line in input.lines() {
        let hand: Hand = Hand::new(line);
        hands.push(hand);
    }

    return hands;
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str =
        "2345A 1
        Q2KJJ 13
        Q2Q2Q 19
        T3T3J 17
        T3Q33 11
        2345J 3
        J345A 2
        32T3K 5
        T55J5 29
        KK677 7
        KTJJT 34
        QQQJA 31
        JJJJJ 37
        JAAAA 43
        AAAAJ 59
        AAAAA 61
        2AAAA 23
        2JJJJ 53
        JJJJ2 41";

    #[test]
    fn test_process() {
        assert_eq!(6839, process(INPUT));
    }

    #[test]
    fn test_hand() {
        let hand: Hand = Hand {
            bet: 13,
            prio: vec![3, 10, 1, 11, 0, 0],
        };
        assert_eq!(hand, Hand::new("Q2KJJ 13"));

        let hand: Hand = Hand {
            bet: 17,
            prio: vec![4, 9, 2, 9, 2, 0],
        };
        assert_eq!(hand, Hand::new("T3T3J 17"));

        let hand: Hand = Hand {
            bet: 19,
            prio: vec![4, 10, 1, 10, 1, 10],
        };
        assert_eq!(hand, Hand::new("Q2Q2Q 19"));
    }
}
