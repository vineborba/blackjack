use std::{io, str::FromStr};

use strum::IntoEnumIterator;

use crate::{
    action::Action,
    card::Card,
    deck::Deck,
    hand::{Hand, HandCondition},
};

#[derive(Debug)]
pub enum PlayerKind {
    Dealer,
    Player,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PlayerStatus {
    Playing,
    Won,
    Standing,
    Lost,
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
    kind: PlayerKind,
    pot: u32,
    status: PlayerStatus,
    pub hands: Vec<Hand>,
}

impl Player {
    pub fn new(name: String, pot: u32, kind: PlayerKind) -> Self {
        Self {
            kind,
            name,
            pot,
            status: PlayerStatus::Playing,
            hands: vec![],
        }
    }

    pub fn new_hand(&mut self, bet: u32, card: Option<Card>) -> Result<(), String> {
        if bet > self.pot {
            return Err(format!("{} is betting more than owned pot!", self.name));
        }
        self.hands.push(Hand::new(false, bet, card));
        Ok(())
    }

    pub fn add_card_to_hand(&mut self, card: Card, current_hand: usize) {
        self.hands[current_hand].add_card_to_hand(card);
    }

    pub fn play(&mut self, current_hand: usize, deck: &mut Deck) -> Result<&PlayerStatus, String> {
        match self.kind {
            PlayerKind::Dealer => self.dealer_play(deck)?,
            PlayerKind::Player => self.player_play(current_hand, deck)?,
        };
        Ok(self.check_condition(current_hand))
    }

    pub fn still_playing(&self) -> bool {
        matches!(self.status, PlayerStatus::Playing)
    }

    fn check_condition(&mut self, current_hand: usize) -> &PlayerStatus {
        if self.status == PlayerStatus::Playing {
            match self.check_hand(current_hand) {
                HandCondition::Under => {
                    self.status = PlayerStatus::Playing;
                }
                HandCondition::Blackjack => {
                    self.status = PlayerStatus::Won;
                }
                HandCondition::Busted => {
                    self.status = PlayerStatus::Lost;
                }
            }
        };
        &self.status
    }

    fn check_hand(&self, current_hand: usize) -> HandCondition {
        self.hands[current_hand].check_hand()
    }

    fn execute_action(
        &mut self,
        current_hand: usize,
        action: Action,
        deck: &mut Deck,
    ) -> Result<(), String> {
        match action {
            Action::Hit => {
                println!("{}: Hit!", self.name);
                let card = deck.deal_card();
                println!("{} got the card {}", self.name, card);
                self.add_card_to_hand(card, current_hand);
            }
            Action::Stand => {
                println!("{}: Stand.", self.name);
                self.status = PlayerStatus::Standing;
            }
            Action::DoubleDown => {
                println!("{}: DOUBLE DOWN!", self.name);
                let card = deck.deal_card();
                println!("{} got the card {}", self.name, card);
                self.add_card_to_hand(card, current_hand);
                self.hands[current_hand].double_bet();
                self.status = PlayerStatus::Standing;
            }
            Action::Split => {
                println!("{}: Split!", self.name);
                let hand = &mut self.hands[current_hand];
                let (card, bet) = hand.split();
                self.new_hand(bet, card)?;
                let card = deck.deal_card();
                println!(
                    "{} got the card {} for their {} hand",
                    self.name,
                    card,
                    current_hand + 1
                );
                self.add_card_to_hand(card, current_hand);
                let card = deck.deal_card();
                println!(
                    "{} got the card {} for their {} hand",
                    self.name,
                    card,
                    self.hands.len()
                );
                self.add_card_to_hand(card, self.hands.len() - 1);
            }
            Action::Surrender => {
                println!("{}: I surrender!", self.name);
                self.status = PlayerStatus::Lost;
            }
        };
        Ok(())
    }

    fn dealer_play(&mut self, deck: &mut Deck) -> Result<(), String> {
        match self.hands[0].sum_value() {
            0..=16 => self.execute_action(0, Action::Hit, deck)?,
            _ => self.execute_action(0, Action::Stand, deck)?,
        };
        Ok(())
    }

    fn player_play(&mut self, current_hand: usize, deck: &mut Deck) -> Result<(), String> {
        let hand = &self.hands[current_hand];
        let hand_size = hand.size();
        let hands_count = self.hands.len();
        let bet = hand.current_bet();
        let cards_are_equal = hand.cards_are_equal();

        let possible_actions: Vec<Action> = Action::iter()
            .filter_map(|action| {
                if action.can_execute(hand_size, hands_count, self.pot, bet, cards_are_equal) {
                    return Some(action);
                }
                None
            })
            .collect();

        println!("{}", "*".repeat(90));

        println!(
            "{}, this is your status for your {} hand:\n{}",
            self.name,
            current_hand + 1,
            self.hands[current_hand]
        );
        println!();

        println!("What will you do?");
        Action::print_actions_list(&possible_actions);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");
        let input = input.trim();

        let action = Action::from_str(input)?;
        self.execute_action(current_hand, action, deck)?;

        println!("{}", "*".repeat(90));

        Ok(())
    }
}
