use std::fmt;

use crate::card::Card;

#[derive(Debug)]
pub struct Hand {
    cards: Vec<Card>,
    bet: u32,
}

pub enum HandCondition {
    Under,
    Blackjack,
    Busted,
}

impl Hand {
    pub fn new(bet: u32, first_card: Option<Card>) -> Self {
        let mut cards = vec![];
        if let Some(card) = first_card {
            cards.push(card);
        }
        Self { bet, cards }
    }

    pub fn add_card_to_hand(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn double_bet(&mut self) {
        self.bet *= 2;
    }

    pub fn sum_value(&self) -> u8 {
        self.cards.iter().fold(0, |mut acc, item| {
            acc += item.value(acc);
            acc
        })
    }

    pub fn size(&self) -> usize {
        self.cards.len()
    }

    pub fn check_hand(&self) -> HandCondition {
        match self.sum_value() {
            0..=20 => HandCondition::Under,
            21 => HandCondition::Blackjack,
            _ => HandCondition::Busted,
        }
    }

    pub fn split(&mut self) -> (Option<Card>, u32) {
        (self.cards.pop(), self.bet)
    }

    pub fn current_bet(&self) -> u32 {
        self.bet
    }

    pub fn cards_are_equal(&self) -> bool {
        self.cards.iter().all(|c| *c == self.cards[0])
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_cards = self
            .cards
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "cards: {}", string_cards)
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.sum_value() == other.sum_value()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_value = self.sum_value();
        let other_value = other.sum_value();

        self_value.partial_cmp(&other_value)
    }
}
