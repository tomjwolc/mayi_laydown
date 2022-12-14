use rand::{thread_rng, Rng};
use std::cmp::Ordering;

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum Card {
    Heart(u8),
    Diamond(u8),
    Club(u8),
    Spade(u8),
    Joker
}

impl Card {
    pub fn from(num: u8, suit: char) -> Self {
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
    pub fn hand_from(cards: Vec<&str>) -> Vec<Self> {
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

    pub fn joker() -> Self {
        Self::Joker
    }

    pub fn get_deck(num_decks: usize) -> Vec<Self> {
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

    pub fn get_rand_hand(num_cards: usize, num_decks: usize) -> Vec<Self> {
        let mut deck = Self::get_deck(num_decks);
        let mut hand = Vec::new();
        let mut rng = thread_rng();

        for _ in 0..num_cards {
            hand.push(deck.remove(rng.gen_range(0..deck.len())))
        }

        hand
    }

    pub fn get_suit(&self) -> char {
        match self {
            Self::Heart(_) => 'H',
            Self::Diamond(_) => 'D',
            Self::Club(_) => 'C',
            Self::Spade(_) => 'S',
            Self::Joker => 'J'
        }
    }

    pub fn get_suit_num(&self) -> u8 {
        match self {
            Self::Heart(_) => 0,
            Self::Diamond(_) => 1,
            Self::Club(_) => 2,
            Self::Spade(_) => 3,
            Self::Joker => 4
        }
    }

    pub fn get_num(&self) -> u8 {
        match self {
            Self::Heart(n) | 
                Self::Diamond(n) |
                Self::Club(n) |
                Self::Spade(n) => *n,
            Self::Joker => 13
        }
    }

    pub fn is_joker(&self) -> bool {
        match self {
            Self::Joker => true,
            _ => false
        }
    }

    pub fn get_next_card(&self) -> Self {
        if self.is_joker() {
            Self::Joker
        } else {
            Self::from((self.get_num() + 1) % 13, self.get_suit())
        }
    }

    pub fn compare(&self, other: &Self) -> isize {
        ((5 * self.get_num() + self.get_suit_num()) as isize) - ((5 * other.get_num() + other.get_suit_num()) as isize)
    }

    pub fn value(&self) -> usize {
        match self.get_num() {
            0 => 15,
            13 => 20,
            9 | 10 | 11 | 12 => 10,
            n => n as usize + 1
        }
    }
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_joker() {
            write!(f, "JJJ")
        } else {
            let num = self.get_num();
            let suit = self.get_suit();

            write!(f, "{}{}{}", if num < 10 {"0"} else {""}, num, suit)
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
