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
    hand: Vec<Card>,
    kind: PlayerKind,
    bet: u32,
    pot: u32,
    status: PlayerStatus,
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

impl Player {
    pub fn new(name: String, bet: u32, kind: PlayerKind) -> Self {
        Self {
            kind,
            name,
            bet,
            pot: bet,
            status: PlayerStatus::Playing,
            hand: vec![],
        }
    }

    pub fn add_card_to_hand(&mut self, card: Card) {
        self.hand.push(card);
    }

    pub fn play(&mut self, deck: &mut Deck) -> &PlayerStatus {
        match self.kind {
            PlayerKind::Dealer => self.dealer_play(deck),
            PlayerKind::Player => self.player_play(deck),
        };
        self.check_condition()
    }

    fn sum_hand(&self) -> u8 {
        self.hand.iter().fold(0, |mut acc, item| {
            acc += item.value(acc);
            acc
        })
    }

    fn check_condition(&mut self) -> &PlayerStatus {
        if self.status == PlayerStatus::Playing {
            match self.check_hand() {
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

    fn check_hand(&self) -> HandCondition {
        match self.sum_hand() {
            0..=20 => HandCondition::Under,
            21 => HandCondition::Blackjack,
            _ => HandCondition::Busted,
        }
    }

    fn take_action(&mut self, play: Play, deck: &mut Deck) {
        match play {
            Play::Hit => {
                println!("{}: Hit!", self.name);
                self.add_card_to_hand(deck.deal_card());
            }
            Play::Stand => {
                println!("{}: Stand.", self.name);
                self.status = PlayerStatus::Standing;
            }
            Play::DoubleDown => {
                println!("{}: DOUBLE DOWN!", self.name);
                self.bet *= 2;
                self.add_card_to_hand(deck.deal_card());
            }
            Play::Split => {
                println!("{}: Split!", self.name);
            }
            Play::Surrender => {
                println!("{}: I surrender!", self.name);
                self.status = PlayerStatus::Lost;
            }
        }
    }

    fn player_play(&mut self, deck: &mut Deck) {}

    fn dealer_play(&mut self, deck: &mut Deck) {
        match self.sum_hand() {
            0..=16 => self.take_action(Play::Hit, deck),
            _ => self.take_action(Play::Stand, deck),
        }
    }
}
