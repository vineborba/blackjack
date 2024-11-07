use crate::{card::Card, deck::Deck};

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

pub struct Player {
    pub name: String,
    kind: PlayerKind,
    pot: u32,
    status: PlayerStatus,
    pub hands: Vec<Hand>,
}

pub enum HandCondition {
    Under,
    Blackjack,
    Busted,
}

pub enum Play {
    Hit,
    Stand,
    DoubleDown,
    Split,
    Surrender,
}

pub struct Hand {
    cards: Vec<Card>,
    splitted: bool,
    bet: u32,
}

impl Hand {
    pub fn new(splitted: bool, bet: u32, first_card: Option<Card>) -> Self {
        let mut cards = vec![];
        if let Some(card) = first_card {
            cards.push(card);
        }
        Self {
            bet,
            splitted,
            cards,
        }
    }

    pub fn add_card_to_hand(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn double_bet(&mut self) {
        self.bet *= 2;
    }

    fn sum_value(&self) -> u8 {
        self.cards.iter().fold(0, |mut acc, item| {
            acc += item.value(acc);
            acc
        })
    }
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
            return Err(format!("{} is betting more than owned pot!", self.name))
        }
        self.hands.push(Hand::new(false, bet, card));
        Ok(())
    }

    pub fn add_card_to_hand(&mut self, card: Card, current_hand: usize) {
        self.hands[current_hand].cards.push(card);
    }

    pub fn play(&mut self, current_hand: usize, deck: &mut Deck) -> &PlayerStatus {
        match self.kind {
            PlayerKind::Dealer => self.dealer_play(deck),
            PlayerKind::Player => self.player_play(current_hand, deck),
        };
        self.check_condition(current_hand)
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
        match self.hands[current_hand].sum_value() {
            0..=20 => HandCondition::Under,
            21 => HandCondition::Blackjack,
            _ => HandCondition::Busted,
        }
    }

    fn take_action(&mut self, current_hand: usize, play: Play, deck: &mut Deck) {
        match play {
            Play::Hit => {
                println!("{}: Hit!", self.name);
                self.add_card_to_hand(deck.deal_card(), current_hand);
            }
            Play::Stand => {
                println!("{}: Stand.", self.name);
                self.status = PlayerStatus::Standing;
            }
            Play::DoubleDown => {
                println!("{}: DOUBLE DOWN!", self.name);
                self.add_card_to_hand(deck.deal_card(), current_hand);
                self.hands[current_hand].double_bet();
                self.status = PlayerStatus::Standing;
            }
            Play::Split => {
                println!("{}: Split!", self.name);
                let card = self.hands[current_hand].cards.pop();
                self.new_hand(self.hands[current_hand].bet, card);
                self.add_card_to_hand(deck.deal_card(), current_hand);
                self.add_card_to_hand(deck.deal_card(), self.hands.len() - 1);
            }
            Play::Surrender => {
                println!("{}: I surrender!", self.name);
                self.status = PlayerStatus::Lost;
            }
        }
    }

    fn player_play(&mut self, current_hand: usize, deck: &mut Deck) {}

    fn dealer_play(&mut self, deck: &mut Deck) {
        match self.hands[0].sum_value() {
            0..=16 => self.take_action(0, Play::Hit, deck),
            _ => self.take_action(0, Play::Stand, deck),
        }
    }
}
