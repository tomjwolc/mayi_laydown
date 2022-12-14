use super::*;

pub fn hand_eval(hand: &Vec<Card>, num_foursies: usize, num_threesies: usize) -> f64 {
    if hand.len() < num_foursies + num_threesies { return -1.0 };

    let laydown_possibilities = get_all_laydown_possibilities(
        num_foursies, 
        num_threesies, 
        &hand
    );

    let mut max_value = 0;
    let mut max_cost = 0;

    for possibility in laydown_possibilities.iter() {
        let (value, cost) = possibility.iter().fold((0, 0), |accum, run| {
            let result = run.iter()
                .fold((0, 0), |accum, card| (accum.0 + 1, accum.1 + card.value()));

            (accum.0 + result.0, accum.1 + result.1)
        });

        if value > max_value { 
            max_value = value;
            max_cost = cost;
        };
    }

    if laydown_possibilities.len() == 0 {
        let mut new_hand = hand.clone();
        new_hand.push(Card::Joker);

        hand_eval(&new_hand, num_foursies, num_threesies) / 10.0
    } else {
        10.0 * max_value as f64 + laydown_possibilities.len() as f64 / 5.0 + max_cost as f64 / 50.0
    }
}