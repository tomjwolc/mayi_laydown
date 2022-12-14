use std::thread;

use mayi_laydown::*;

#[test]
fn test_get_foursies() {
    let hand = Card::get_rand_hand(10, 1);
    let foursies = get_foursies(&hand, &0);

    println!("Hand: {:?}", hand);
    println!("Foursies: {:?}", foursies);
}

#[test]
fn test_get_threesies() {
    let hand = Card::hand_from(vec![
        "03S",
        "03D",
        "03H",
        "03S",
        "03D"
    ]);

    println!("Hand: {:?}\nThreesies: {:?}", hand, get_threesies(&hand, &0))
}

#[test]
fn test_possibilities() {
    let hand = Card::get_rand_hand(20, 2);

    let laydown_possibilities = get_all_laydown_possibilities(0, 2, &hand);
    let mut string_possibilities = laydown_possibilities
        .iter()
        .map(|possibility| format!(
            "{:04} | {:?}",
            possibility.iter().fold(0, |accum, run| {
               accum + run.iter().fold(0, |accum, card| accum + card.value())
            }),
            possibility
        ))
        .collect::<Vec<String>>();

    string_possibilities.sort();

    println!(
        "Hand: {:?}\nLaydown Possibilities: [\n    {}\n]", 
        hand, 
        string_possibilities.join("\n    ")
    );
}

#[test]
fn test_lots_of_jokers() {
    let mut hand = Card::get_rand_hand(20, 2);
    hand.push(Card::Joker);
    hand.push(Card::Joker);
    hand.push(Card::Joker);

    let laydown_possibilities = get_all_laydown_possibilities(2, 2, &hand);
    let mut string_possibilities = laydown_possibilities
        .iter()
        .map(|possibility| format!(
            "{:04} | {:?}",
            possibility.iter().fold(0, |accum, run| {
               accum + run.iter().fold(0, |accum, card| accum + card.value())
            }),
            possibility
        ))
        .collect::<Vec<String>>();

    string_possibilities.sort();

    println!(
        "Hand: {:?}\nLaydown Possibilities: [\n    {}\n]", 
        hand, 
        string_possibilities.join("\n    ")
    );
}

#[test]
fn test_hand_eval() {
    let mut strings = Vec::new();

    for _ in 0..100000 {
        let hand = Card::get_rand_hand(10, 2);
        let mut possibilities = get_all_laydown_possibilities(1, 1, &hand);

        possibilities.sort_by(|pos1, pos2| {
            pos2.iter().fold(0, |accum, run| accum + run.len()).partial_cmp(
                &pos1.iter().fold(0, |accum, run| accum + run.len())
            ).unwrap()
        });

        strings.push(format!("{:07.3} || {:?} || {:?}", hand_eval(&hand, 1, 1), hand, possibilities.get(0)));
    }

    strings.sort();

    for (i, string) in strings.iter().enumerate() {
        println!("{:7}.) {}", strings.len() - i, string);
    }
}

#[test]
fn test_percents() {
    output_insta_laydown_percents(100000, 2);
}

#[allow(dead_code)]
fn output_insta_laydown_percents(sims: usize, num_decks: usize) {
    let rounds = vec![
        (1, 1, 10),
        (2, 0, 10),
        (0, 3, 10),
        (1, 2, 14),
        (2, 1, 14),
        (3, 0, 14)
    ];

    for round in rounds {
        let num_threads = 10;
        let mut handles = Vec::new();

        for _ in 0..num_threads {
            handles.push(thread::spawn(move || {
                let mut num_instant_laydowns = 0;

                for _ in 0..(sims / num_threads) {
                    let possible_laydowns = get_all_laydown_possibilities(
                        round.0, 
                        round.1, 
                        &Card::get_rand_hand(round.2, num_decks)
                    );
        
                    if possible_laydowns.len() > 0 {
                        num_instant_laydowns += 1;
                    }
                }

                num_instant_laydowns
            }));
        }

        let mut result: usize = 0;

        for handle in handles {
            result += handle.join().unwrap();
        }

        let mut round_name = format!("{}{}", "3, ".repeat(round.1), "4, ".repeat(round.0));
        round_name.pop();
        round_name.pop();

        println!(
            "Instant Laydown Percentage {:.3}%  |  Round: {}", 
            100.0 * result as f64 / sims as f64,
            round_name
        );
    }
}