use std::collections::VecDeque;

use crate::card::{Card, Figure, Suit};
use rand::{seq::SliceRandom, thread_rng};
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
    shuffled_cards: VecDeque<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(52);
        for s in Suit::iter() {
            for f in Figure::iter() {
                cards.push(Card::new(s, f));
            }
        }

        Self {
            cards,
            shuffled_cards: VecDeque::new(),
        }
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);

        if !self.shuffled_cards.is_empty() {
            self.shuffled_cards = VecDeque::new();
        }

        for card in &self.cards {
            self.shuffled_cards.push_back(card.clone());
        }
    }

    pub fn deal_card(&mut self) -> Card {
        if let Some(card) = self.shuffled_cards.pop_back() {
            card
        } else {
            unreachable!()
        }
    }
}
