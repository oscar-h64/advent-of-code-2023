use std::collections::{HashMap, HashSet};

fn parse_num_list(list: &str) -> HashSet<u64> {
    HashSet::from_iter(list
        .split_whitespace()
        .map(|n| n.parse::<u64>().expect("should parse number")))
}

pub fn a(inp: &str) -> u64 {

    inp.split('\n').map(|line| {

        // Split into: Card prefix, winning numbers, numbers
        let mut parts = line.split(&[':', '|']);

        // discord the card prefix
        parts.next();

        // Parse the winning numbers and what we have
        let winning_numbers = parse_num_list(parts.next().expect("should have winning numbers"));
        let numbers = parse_num_list(parts.next().expect("should have card numbers"));

        let overlap = winning_numbers.intersection(&numbers).count();

        if overlap > 0 {
            // 1 is worth 1 not 2
            1 << overlap - 1
        } else {
            0
        }
    }).sum()
}

pub fn b(inp: &str) -> u64 {

    // Important note: this works, as once we get to a card, we never change the number we have of it
    // or of any cards before it

    // Keep track of the card number, and the extra number of each card
    let mut hash_map: HashMap<u64, u64> = HashMap::new();
    let mut card_number: u64 = 0;

    inp.split('\n').map(|line| {

        card_number += 1;

        // Split into: Card prefix, winning numbers, numbers
        let mut parts = line.split(&[':', '|']);

        // discord the card prefix
        parts.next();

        // Parse the winning numbers and what we have
        let winning_numbers = parse_num_list(parts.next().expect("should have winning numbers"));
        let numbers = parse_num_list(parts.next().expect("should have card numbers"));

        // Calculate the number of overlapping numbers
        let overlap = winning_numbers.intersection(&numbers).count();

        // See how many of this card we have
        let num_of_this_card = 1 + hash_map.get(&card_number).unwrap_or(&0);

        // For each card of the next n cards (n = num winning numbers on this one), increase the
        // number of those cards we have by the number of this card (as we get one extra of those
        // for each of this card)
        let lower_bound = card_number + 1;
        let upper_bound = card_number.clone() + 1 + (overlap as u64);
        for i in lower_bound..upper_bound {
            hash_map.entry(i)
                .and_modify(|x| *x += num_of_this_card.clone())
                .or_insert(num_of_this_card.clone());
        }

        // Return the number of this card
        num_of_this_card
    }).sum()
}
