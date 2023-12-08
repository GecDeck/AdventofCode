const CARD_PRIO: &[char; 13] = &['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
const HAND_TYPES: &[&[u32]; 7] =
    &[&[1, 1, 1, 1, 1], &[1, 1, 1, 2, 2], &[1, 2, 2, 2, 2], &[1, 1, 3, 3, 3],
    &[2, 2, 3, 3, 3], &[1, 4, 4, 4, 4], &[5, 5, 5, 5, 5]];
// Representation of each type of hand in order of lowest to highest prio

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

    return total_winnings;
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
        let mut hand_type: Vec<u32> = vec![];
        for card in &cards {
            let mut dupes: u32 = 0;
            // dupes will always be at least 1
            for dupe_card in &cards {
                if card == dupe_card {
                    dupes += 1;
                }
            }
            hand_type.push(dupes);
        }
        hand_type.sort();

        for i in 0..HAND_TYPES.len() {
            if hand_type == HAND_TYPES[i] {
                prio.insert(0, i as u32);
            }
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
        "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483";

    #[test]
    fn test_process() {
        assert_eq!(6440, process(INPUT));
    }

    #[test]
    fn test_hand() {
        let hand: Hand = Hand {
            bet: 765,
            prio: vec![1, 1, 0, 8, 1, 11],
        };
        assert_eq!(hand, Hand::new("32T3K 765"));
    }
}
