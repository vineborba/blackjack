use strum::EnumIter;

#[derive(Debug, EnumIter, PartialEq, Clone, Copy)]
pub enum Suit {
    Hearts,
    Spades,
    Clubs,
    Diamods,
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

#[derive(Debug, Clone, Copy)]
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
