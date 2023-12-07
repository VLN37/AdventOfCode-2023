use itertools::Itertools;

#[derive(Hash, Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Card {
    Two   = 2,
    Three = 3,
    Four  = 4,
    Five  = 5,
    Six   = 6,
    Seven = 7,
    Eight = 8,
    Nine  = 9,
    Ten   = 10,
    Jack  = 11,
    Queen = 12,
    King  = 13,
    #[default]
    Ace   = 14,
}

impl From<u8> for Card {
    fn from(value: u8) -> Self {
        match value {
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            7 => Self::Seven,
            8 => Self::Eight,
            9 => Self::Nine,
            10 => Self::Ten,
            11 => Self::Jack,
            12 => Self::Queen,
            13 => Self::King,
            14 => Self::Ace,
            _ => panic!("Invalid Card: {value}"),
        }
    }
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!("Invalid Card: {value}"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Default)]
pub enum Hand {
    #[default]
    HighCard,
    OnePair,
    TwoPair,
    ThreeOf,
    FullHouse,
    FourOf,
    FiveOf,
}

#[derive(Ord, Eq, PartialEq, PartialOrd, Debug, Default)]
pub struct Game {
    pub value: Hand,
    pub cards: [Card; 5],
    pub bid:   usize,
}

impl From<Vec<u8>> for Game {
    fn from(v: Vec<u8>) -> Self {
        let mut arr = [Card::Ace; 5];
        for i in 0..v.len() {
            arr[i] = v[i].into();
        }
        Game {
            cards: arr,
            ..Self::default()
        }
    }
}

impl From<Vec<char>> for Game {
    fn from(v: Vec<char>) -> Self {
        let mut arr = [Card::Ace; 5];
        for i in 0..v.len() {
            arr[i] = v[i].into();
        }
        Game {
            cards: arr,
            ..Self::default()
        }
    }
}

impl Game {
    pub fn compute(&mut self) {
        let uniques: Vec<Card> = self.cards.iter().unique().cloned().collect();
        let mut qtys: Vec<u8> = Vec::new();
        for unique in &uniques {
            let n = self.cards.iter().cloned().filter(|x| *x == *unique).count();
            qtys.push(n as u8);
        }
        // for (unique, qty) in zip(uniques, qtys) {
        //     println!("{unique:?}: {qty}");
        // }
        let hand = match qtys.iter().max().unwrap() {
            5 => Hand::FiveOf,
            4 => Hand::FourOf,
            3 => {
                if qtys.iter().cloned().filter(|x| *x == 2).count() == 1 {
                    Hand::FullHouse
                } else {
                    Hand::ThreeOf
                }
            }
            2 => {
                if qtys.iter().cloned().filter(|x| *x == 2).count() == 2 {
                    Hand::TwoPair
                } else {
                    Hand::OnePair
                }
            }
            _ => Hand::HighCard,
        };
        self.value = hand;
    }
}
