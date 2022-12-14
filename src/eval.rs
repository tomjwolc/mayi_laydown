use super::*;

pub fn hand_eval(hand: &Vec<Card>, num_foursies: usize, num_threesies: usize) -> f64 {
    if hand.len() < num_foursies + num_threesies { return -1.0 };

    let laydown_possibilities = get_all_laydown_possibilities(
        num_foursies, 
        num_threesies, 
        &hand
    );

    let mut max_value = 0;

    for possibility in laydown_possibilities.iter() {
        let value = possibility.iter().fold(0, |accum, run| {
            accum + run.iter().fold(0, |accum, _card| accum + 1)
        });

        if value > max_value { max_value = value };
    }

    if laydown_possibilities.len() == 0 {
        let mut new_hand = hand.clone();
        new_hand.push(Card::Joker);

        hand_eval(&new_hand, num_foursies, num_threesies) / 10.0
    } else {
        (10 * max_value + laydown_possibilities.len() / 5) as f64
    }
}