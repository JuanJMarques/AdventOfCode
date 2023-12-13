use std::cmp::Ordering;
use std::fs::File;
use std::io::Read;

#[derive(Eq, PartialEq, Clone, Debug)]
struct Hand {
    cards: Vec<char>,
    strength: u32,
    bet: u64,
}
fn main() {
    let total_winnings = calculate_total_winnings(read_input_file());
    println!("the total winning is {}", total_winnings);
}

fn read_input_file() -> String {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    content
}

fn calculate_total_winnings(input: String) -> u64 {
    let mut hands = input.lines().map(Hand::new).collect::<Vec<Hand>>();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) as u64 * hand.bet)
        .sum()
}

impl Hand {
    fn new(line: &str) -> Hand {
        let asd = line
            .split(' ')
            .map(|s| s.trim())
            .filter(|&s| !s.is_empty())
            .collect::<Vec<&str>>();
        Hand {
            cards: get_individual_cards(asd[0]),
            strength: calculate_hand_strength(asd[0]),
            bet: asd[1].parse::<u64>().unwrap(),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.strength.cmp(&other.strength) {
            Ordering::Equal => compare_cards(&self.cards, &other.cards),
            order => Some(order),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.strength.cmp(&other.strength) {
            Ordering::Equal => compare_cards(&self.cards, &other.cards).unwrap(),
            order => order,
        }
    }
}

fn compare_cards(self_cards: &[char], other_cards: &[char]) -> Option<Ordering> {
    if self_cards.is_empty() || other_cards.is_empty() {
        return Some(Ordering::Equal);
    }
    let self_card_strength = calculate_card_strength(self_cards[0]);
    let other_card_strength = calculate_card_strength(other_cards[0]);
    match self_card_strength.cmp(&other_card_strength) {
        Ordering::Equal => compare_cards(&self_cards[1..], &other_cards[1..]),
        order => Some(order),
    }
}

fn calculate_hand_strength(cards: &str) -> u32 {
    let cards = get_individual_cards(cards);
    if is_five_of_a_kind(&cards) {
        return 7;
    }
    if is_n_of_a_kind(&cards, 4) {
        return 6;
    }
    if is_full_house(&cards) {
        return 5;
    }
    if is_n_of_a_kind(&cards, 3) {
        return 4;
    }
    if is_two_pairs(&cards) {
        return 3;
    }
    if is_n_of_a_kind(&cards, 2) {
        return 2;
    }
    1
}

fn calculate_card_strength(card: char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => card.to_digit(10).unwrap(),
    }
}

fn is_five_of_a_kind(cards: &[char]) -> bool {
    let reference = cards[0];
    cards.iter().all(|&s| s == reference)
}

fn is_n_of_a_kind(cards: &[char], n: usize) -> bool {
    if cards.len() < n {
        return false;
    }
    let reference = cards[0];
    if cards.iter().filter(|&&s| s == reference).count() < n {
        return is_n_of_a_kind(&cards[1..], n);
    }
    true
}

fn is_n_of_a_kind_strict(cards: &[char], n: usize, reference: char) -> bool {
    if cards.len() < n {
        return false;
    }
    if cards.iter().filter(|&&s| s == reference).count() < n {
        let asd = &cards[1..];
        return is_n_of_a_kind_strict(asd, n, reference);
    }
    true
}

fn is_full_house(cards: &[char]) -> bool {
    let reference = cards[0];
    if is_n_of_a_kind_strict(cards, 3, reference) {
        return is_n_of_a_kind(
            &cards
                .iter()
                .filter(|&&c| c != reference)
                .copied()
                .collect::<Vec<char>>(),
            2,
        );
    }
    if is_n_of_a_kind_strict(cards, 2, reference) {
        return is_n_of_a_kind(
            &cards
                .iter()
                .filter(|&&c| c != reference)
                .copied()
                .collect::<Vec<char>>(),
            3,
        );
    }
    false
}

fn is_two_pairs(cards: &[char]) -> bool {
    if cards.len() < 4 {
        return false;
    }
    let reference = cards[0];
    if is_n_of_a_kind_strict(cards, 2, reference) {
        return is_n_of_a_kind(
            &cards
                .iter()
                .filter(|&&c| c != reference)
                .copied()
                .collect::<Vec<char>>(),
            2,
        );
    }
    is_two_pairs(&cards[1..])
}

fn get_individual_cards(cards: &str) -> Vec<char> {
    cards.chars().collect::<Vec<char>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            .to_string();
        let result = calculate_total_winnings(input);
        assert_eq!(result, 6440);
    }
}
