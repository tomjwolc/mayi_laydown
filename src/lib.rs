use std::collections::HashSet;

pub mod card;
pub use card::Card;

pub mod eval;
pub use eval::*;

pub fn get_all_laydown_possibilities(num_foursies: usize, num_threesies: usize, hand: &Vec<Card>) -> Vec<Vec<Vec<Card>>> {
    get_all_laydown_possibilities_helper(
        num_foursies, 
        num_threesies, 
        hand, 
        &0,
        &0
    )
}

fn get_all_laydown_possibilities_helper(
    num_foursies: usize, 
    num_threesies: usize, 
    hand: &Vec<Card>, 
    min_suit_num: &u8,
    min_num: &u8
) -> Vec<Vec<Vec<Card>>> {
    let mut laydown_possibilities = Vec::new(); // vector to store laydown possibilities

    if num_foursies + num_threesies > 0 { // if there are any foursies or threesies required in the laydown
        let runs = if num_foursies > 0 { // if there are foursies required, find all possible foursies in the hand
            get_foursies(&hand, &min_suit_num)
        } else { // otherwise, find all possible threesies in the hand
            get_threesies(&hand, &min_num)
        };
        
        // For each run, get all laydown possibilities once, that run is got
        for run in runs {
            let mut new_min_suit_num = *min_suit_num; // create a copy of the minimum suit number for the recursive call
            let mut new_min_num = *min_num; // create a copy of the minimum number for the recursive call
            let mut new_hand = hand.clone(); // create a copy of the hand for the recursive call
            let mut new_num_foursies = num_foursies; // create a copy of the number of foursies required for the recursive call
            let mut new_num_threesies = num_threesies; // create a copy of the number of threesies required for the recursive call
            
            if num_foursies > 0 { // if the current run is a foursy
                new_min_suit_num = run[0].get_suit_num(); // set the new minimum suit number to be that of run[0]
                new_num_foursies -= 1; // decrement the number of foursies required
            } else { // if the current run is a threesy
                new_min_num = run[0].get_num();
                new_num_threesies -= 1;
            }

            for card in run.iter() {
                let index = new_hand.iter().position(|other_card| other_card == card).unwrap();
                new_hand.remove(index);
            }

            // get all possibilities with the run
            let more_laydown_possibilities = get_all_laydown_possibilities_helper(
                new_num_foursies, 
                new_num_threesies, 
                &new_hand,
                &new_min_suit_num,
                &new_min_num
            );

            // add the foursy to each possibility, and add it to the list of all possibilities
            for mut laydown_possibility in more_laydown_possibilities {
                laydown_possibility.insert(0, run.clone());
                laydown_possibilities.push(laydown_possibility);
            }
        }
    } else {
        laydown_possibilities.push(Vec::new());
    }

    laydown_possibilities
}

pub fn get_foursies(
    hand: &Vec<Card>,
    min_suit_num: &u8 // Imposes an artificial order to cut down on the number of permutations
) -> Vec<Vec<Card>> {
    get_runs(
        hand, 
        4,
        &|card| !card.is_joker() && card.get_suit_num() + 1 > *min_suit_num, 
        &|last_card, next_card| next_card.is_joker() || &last_card.get_next_card() == next_card, 
        &|last_card| last_card.get_next_card()
    )
}

pub fn get_threesies(
    hand: &Vec<Card>,
    min_num: &u8 // Imposes an artificial order to cut down on the number of permutations
) -> Vec<Vec<Card>> {
    get_runs(
        hand, 
        3,
        &|card| !card.is_joker() && card.get_num() + 1 > *min_num, 
        &|last_card, next_card| {
            next_card.is_joker() || (
                last_card.get_num() == next_card.get_num() &&
                last_card.get_suit_num() <= next_card.get_suit_num()
            )
        }, 
        &|last_card| last_card.clone()
    )
}

fn get_runs(
    hand: &Vec<Card>,
    min_length: usize,
    can_start_run: &impl Fn(&Card) -> bool,
    is_next: &impl Fn(&Card, &Card) -> bool,
    replace_joker: &impl Fn(&Card) -> Card
) -> Vec<Vec<Card>> { 
    let mut runs = Vec::new(); 

    for card in hand.iter().collect::<HashSet<&Card>>() {  
        if !can_start_run(card) { continue; }

        let mut my_hand = hand.clone(); 
        let index =  my_hand.iter().position(|other_card| other_card == card).unwrap();
        my_hand.remove(index);

        let my_runs = get_runs_helper(&my_hand, card, is_next, replace_joker);

        for mut my_run in my_runs {  
            my_run.insert(0, card.clone());  
            
            if my_run.len() >= min_length { 
                runs.push(my_run);  
            }
        }
    }

    runs  // Return the list of four-of-a-kind hands
}

fn get_runs_helper(
    hand: &Vec<Card>,
    last_card: &Card,
    is_next: &impl Fn(&Card, &Card) -> bool,
    replace_joker: &impl Fn(&Card) -> Card
) -> Vec<Vec<Card>> {
    let mut runs = vec![Vec::new()];

    for card in hand.iter().collect::<HashSet<&Card>>() {
        if !is_next(last_card, card) { continue; }
        
        let mut my_hand = hand.clone();
        let index =  my_hand.iter().position(|other_card| other_card == card).unwrap();
        my_hand.remove(index);

        let my_runs = get_runs_helper(
            &my_hand, 
            &(if card.is_joker() { 
                replace_joker(last_card)
            } else {
                card.clone()
            }),
            is_next,
            replace_joker
        );

        for mut my_run in my_runs {
            my_run.insert(0, card.clone());
            runs.push(my_run);
        }
    }

    runs
}