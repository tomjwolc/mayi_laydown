use rand::{thread_rng, Rng};
use std::{collections::HashSet, cmp::Ordering, thread};

#[derive(Clone, Hash, PartialEq, Eq)]
enum Card {
    Heart(u8),
    Diamond(u8),
    Club(u8),
    Spade(u8),
    Joker
}

impl Card {
    fn from(num: u8, suit: char) -> Self {
        if num > 12 { panic!("num doesn't exist"); }

        match suit {
            'H' | 'h' => Self::Heart(num),
            'D' | 'd' => Self::Diamond(num),
            'C' | 'c' => Self::Club(num),
            'S' | 's' => Self::Spade(num),
            'J' | 'j' => Self::Joker,
            _ => panic!("suit doesn't exist")
        }
    }

    // Enter in the form of vec!["05D", "JJJ"]
    fn hand_from(cards: Vec<&str>) -> Vec<Self> {
        let mut hand = Vec::new();

        for card_str in cards {
            if card_str == "JJJ" {
                hand.push(Self::joker());
                continue;
            }

            match (card_str[0..2].parse::<u8>(), card_str[2..3].parse::<char>()) {
                (Ok(num), Ok(suit)) => hand.push(Self::from(num, suit)),
                _ => {}
            }
        }

        hand
    }

    fn joker() -> Self {
        Self::Joker
    }

    fn get_deck(num_decks: usize) -> Vec<Self> {
        let mut deck = Vec::new();

        for _ in 0..num_decks {
            for suit in vec!['H', 'D', 'S', 'C'] {
                for num in 0..12 {
                    deck.push(Self::from(num, suit));
                }
            }

            deck.push(Self::joker());
            deck.push(Self::joker());
        }

        deck
    }

    fn get_rand_hand(num_cards: usize, num_decks: usize) -> Vec<Self> {
        let mut deck = Self::get_deck(num_decks);
        let mut hand = Vec::new();
        let mut rng = thread_rng();

        for _ in 0..num_cards {
            hand.push(deck.remove(rng.gen_range(0..deck.len())))
        }

        hand
    }

    fn get_suit(&self) -> char {
        match self {
            Self::Heart(_) => 'H',
            Self::Diamond(_) => 'D',
            Self::Club(_) => 'C',
            Self::Spade(_) => 'S',
            Self::Joker => 'J'
        }
    }

    fn get_suit_num(&self) -> u8 {
        match self {
            Self::Heart(_) => 0,
            Self::Diamond(_) => 1,
            Self::Club(_) => 2,
            Self::Spade(_) => 3,
            Self::Joker => 4
        }
    }

    fn get_num(&self) -> u8 {
        match self {
            Self::Heart(n) | 
                Self::Diamond(n) |
                Self::Club(n) |
                Self::Spade(n) => *n,
            Self::Joker => 13
        }
    }

    fn is_joker(&self) -> bool {
        match self {
            Self::Joker => true,
            _ => false
        }
    }

    fn get_next_card(&self) -> Self {
        if self.is_joker() {
            Self::Joker
        } else {
            Self::from((self.get_num() + 1) % 13, self.get_suit())
        }
    }

    fn compare(&self, other: &Self) -> isize {
        ((5 * self.get_num() + self.get_suit_num()) as isize) - ((5 * other.get_num() + other.get_suit_num()) as isize)
    }
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_joker() {
            write!(f, "\"JJJ\"")
        } else {
            let num = self.get_num();
            let suit = self.get_suit();

            write!(f, "\"{}{}{}\"", if num < 10 {"0"} else {""}, num, suit)
        }
    }
}

impl std::cmp::PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self.compare(other) {
            n if n < 0 => Ordering::Less,
            n if n > 0 => Ordering::Greater,
            _ => Ordering::Equal
        })
    }
}

impl std::cmp::Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    let hand = Card::hand_from(vec![
        "03D",
        "12D",
        "00D",
        "JJJ",
        "01D",
        "11D",
        "07H",
        "10H",
        "08H",
        "03S",
        "03D"
    ]);

    println!(
        "Hand: {:?}\n\nfoursies: {:?}\n\nPossibilities: {:?}", 
        hand,
        get_foursies(&hand, &HashSet::new()),
        get_all_laydown_possibilities(2, 1, &hand)
    );
}

fn get_all_laydown_possibilities(num_foursies: usize, num_threesies: usize, hand: &Vec<Card>) -> Vec<Vec<Vec<Card>>> {
    println!("{{\"next\": [");
    let x = get_all_laydown_possibilities_helper(
        num_foursies, 
        num_threesies, 
        hand, 
        &HashSet::new(),
        &HashSet::new()
    );
    println!("0]}}");

    x
}

fn get_all_laydown_possibilities_helper(
    num_foursies: usize, 
    num_threesies: usize, 
    hand: &Vec<Card>, 
    banned_suits: &HashSet<char>,
    banned_nums: &HashSet<u8>
) -> Vec<Vec<Vec<Card>>> {
    let mut laydown_possibilities = Vec::new();

    if num_foursies + num_threesies > 0 {
        let runs = if num_foursies > 0 {
            get_foursies(&hand, &banned_suits)
        } else {
            get_threesies(&hand, &banned_nums)
        };
        
        // For each run, get all laydown possibilities once, that run is got
        for run in runs {
            let mut new_banned_suits = banned_suits.clone();
            let mut new_banned_nums = banned_nums.clone();
            let mut new_hand = hand.clone();
            let mut new_num_foursies = num_foursies;
            let mut new_num_threesies = num_threesies;
            
            if num_foursies > 0 {
                new_banned_suits.insert(new_hand[0].get_suit());
                new_num_foursies -= 1;
            } else {
                new_banned_nums.insert(new_hand[0].get_num());
                new_num_threesies -= 1;
            }

            for card in run.iter() {
                let index = new_hand.iter().position(|other_card| other_card == card).unwrap();
                new_hand.remove(index);
            }

            println!("{{");

            println!("\n\"hand\": {:?},\n\"new_hand\": {:?},\n\"run\": {:?},\n\"banned_suits\": {:?},\n\"banned_nums\": {:?},\n\"next\": [", hand, new_hand, run, new_banned_suits.iter().map(|c| c.to_string()).collect::<Vec<String>>(), new_banned_nums.iter().collect::<Vec<&u8>>());

            // get all possibilities with the run
            let more_laydown_possibilities = get_all_laydown_possibilities_helper(
                new_num_foursies, 
                new_num_threesies, 
                &new_hand,
                &new_banned_suits,
                &new_banned_nums
            );

            println!("0]}},");

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


fn get_foursies(
    hand: &Vec<Card>,
    banned_suits: &HashSet<char>
) -> Vec<Vec<Card>> {
    get_runs(
        hand, 
        4,
        &|card| !card.is_joker() && !banned_suits.contains(&card.get_suit()), 
        &|last_card, next_card| next_card.is_joker() || &last_card.get_next_card() == next_card, 
        &|last_card| last_card.get_next_card()
    )
}

#[test]
fn test_threesies() {
    let hand = Card::hand_from(vec![
        "03S",
        "03D",
        "03H",
        "03S",
        "03D"
    ]);

    println!("Hand: {:?}\nThreesies: {:?}", hand, get_threesies(&hand, &HashSet::new()))
}

fn get_threesies(
    hand: &Vec<Card>,
    banned_nums: &HashSet<u8>
) -> Vec<Vec<Card>> {
    get_runs(
        hand, 
        3,
        &|card| !card.is_joker() && !banned_nums.contains(&card.get_num()), 
        &|last_card, next_card| {
            next_card.is_joker() || (
                last_card.get_num() == next_card.get_num() &&
                // Impose an order to reduce number of permutations
                last_card.get_suit_num() >= next_card.get_suit_num()
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

#[test]
fn test_get_foursies() {
    let hand = Card::get_rand_hand(10, 1);
    let foursies = get_foursies(&hand, &HashSet::new());

    println!("Hand: {:?}", hand);
    println!("Foursies: {:?}", foursies);
}

#[test]
fn see_insta_laydown_percents() {
    output_insta_laydown_percents(10000000, 2);
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
        let num_threads = 20;
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