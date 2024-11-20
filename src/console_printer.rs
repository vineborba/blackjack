use crate::{
    action::Action,
    player::Player,
    printer::{Generic, Message, Printer, Result},
};

pub struct ConsolePrinter {
    round: u16,
}

impl ConsolePrinter {
    pub fn new(round: u16) -> Self {
        Self { round }
    }

    pub fn print_divider(&self) {
        println!("{}", "*".repeat(90));
    }
}

impl Printer for ConsolePrinter {
    fn set_round_settings(&mut self, round: u16) {
        self.round = round;
    }

    fn print_message(&self, message: Message, player: Option<&Player>) {
        match message {
            Message::PlayerStatus => {
                let player = player.expect("Did not pass a valid player ref to Printer");
                match player.kind {
                    crate::player::PlayerKind::Dealer => {
                        self.print_divider();
                        println!("The dealer has the following hand:\n{}", player.hands[0]);
                        println!("{}", "*".repeat(90));
                    }
                    crate::player::PlayerKind::Player => todo!(),
                }
            }
            Message::Action(a) => match a {
                Action::Hit => todo!(),
                Action::Stand => todo!(),
                Action::DoubleDown => todo!(),
                Action::Split => todo!(),
                Action::Surrender => todo!(),
            },
            Message::Result(r) => match r {
                Result::Tie => todo!(),
                Result::DealerWon => todo!(),
                Result::PlayerWon => todo!(),
                Result::Blackjack => todo!(),
            },
            Message::Generic(m) => match m {
                Generic::VerifyResults => {
                    println!("Verifying results now");
                }
                Generic::Shuffling => {
                    println!("Shuffling deck...\n");
                }
                Generic::Starting => {
                    println!("Starting now round {}!\n\n", self.round);
                }
                Generic::Exiting => {
                    println!("All rounds were played, exiting...");
                }
            },
        }
    }
}
