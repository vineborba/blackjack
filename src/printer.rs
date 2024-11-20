use crate::{action::Action, player::Player};

pub enum Result {
    Tie,
    DealerWon,
    PlayerWon,
    Blackjack,
}

pub enum Generic {
    VerifyResults,
    Shuffling,
    Starting,
    Exiting,
}

pub enum Message {
    PlayerStatus,
    Generic(Generic),
    Action(Action),
    Result(Result),
}

pub trait Printer {
    fn print_message(&self, message: Message, player: Option<&Player>);

    fn set_round_settings(&mut self, round: u16);
}
