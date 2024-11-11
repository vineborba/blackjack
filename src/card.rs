use std::fmt;

use strum::EnumIter;

#[derive(Debug, EnumIter, PartialEq, Clone, Copy)]
pub enum Suit {
    Hearts,
    Spades,
    Clubs,
    Diamods,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Suit::Hearts => write!(f, "\u{2665} "),
            Suit::Spades => write!(f, "\u{2660}"),
            Suit::Clubs => write!(f, "\u{2663}"),
            Suit::Diamods => write!(f, "\u{2666}"),
        }
    }
}

#[derive(Debug, EnumIter, PartialEq, Clone, Copy)]
pub enum Figure {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Figure {
    pub fn value(&self, hand_sum: u8) -> u8 {
        match self {
            Figure::Two => 2,
            Figure::Three => 3,
            Figure::Four => 4,
            Figure::Five => 5,
            Figure::Six => 6,
            Figure::Seven => 7,
            Figure::Eight => 8,
            Figure::Nine => 9,
            Figure::Ten | Figure::Jack | Figure::Queen | Figure::King => 10,
            Figure::Ace => {
                if hand_sum >= 11 {
                    1
                } else {
                    11
                }
            }
        }
    }
}

impl fmt::Display for Figure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Figure::Two => write!(f, "2"),
            Figure::Three => write!(f, "3"),
            Figure::Four => write!(f, "4"),
            Figure::Five => write!(f, "5"),
            Figure::Six => write!(f, "6"),
            Figure::Seven => write!(f, "7"),
            Figure::Eight => write!(f, "8"),
            Figure::Nine => write!(f, "9"),
            Figure::Ten => write!(f, "10"),
            Figure::Jack => write!(f, "J"),
            Figure::Queen => write!(f, "Q"),
            Figure::King => write!(f, "K"),
            Figure::Ace => write!(f, "A"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Card {
    #[allow(unused)]
    suit: Suit,
    figure: Figure,
}

impl Card {
    pub fn new(suit: Suit, figure: Figure) -> Self {
        Self { suit, figure }
    }

    pub fn value(&self, hand_sum: u8) -> u8 {
        self.figure.value(hand_sum)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.figure, self.suit)
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.figure == other.figure
    }
}

impl Eq for Card {}
