use crate::{
    deck::Deck,
    hand::HandCondition,
    player::{Player, PlayerKind},
    printer::{Generic, Message, Printer},
};

#[derive(Debug)]
pub struct Game<T: Printer> {
    number_of_rounds: u16,
    played_rounds: u16,
    players: Vec<Player>,
    dealer: Player,
    deck: Deck,
    printer: T,
}

impl<T: Printer> Game<T> {
    pub fn new(_number_of_rounds: u16, printer: T) -> Self {
        Self {
            printer,
            number_of_rounds: 1,
            played_rounds: 0,
            players: vec![],
            deck: Deck::new(),
            dealer: Player::new("Dealer".into(), 0, PlayerKind::Dealer),
        }
    }

    pub fn start(&mut self, players_count: u8) {
        while self.played_rounds < self.number_of_rounds {
            self.setup_round(players_count);
            self.run_round().unwrap_or_else(|e| {
                panic!("Failed to run round {}: {}", self.number_of_rounds + 1, e)
            });
            self.played_rounds += 1;
        }
    }

    fn setup_round(&mut self, players_count: u8) {
        self.printer.set_round_settings(self.played_rounds + 1);
        self.players = vec![];
        for i in 0..players_count {
            let mut player = Player::new(format!("Player {}", i + 1), 100, PlayerKind::Player);
            let mut create_result = player.new_hand(20, None);
            while create_result.is_err() {
                create_result = player.new_hand(10, None)
            }
            self.players.push(player);
        }
        match self.dealer.new_hand(0, None) {
            Ok(_) => (),
            Err(_) => unreachable!(),
        };
    }

    fn run_round(&mut self) -> Result<(), String> {
        self.printer
            .print_message(Message::Generic(Generic::Starting), None);

        self.printer
            .print_message(Message::Generic(Generic::Shuffling), None);

        self.deck.shuffle();

        for _ in 0..2 {
            self.dealer.add_card_to_hand(self.deck.deal_card(), 0);
            for p in self.players.iter_mut() {
                p.add_card_to_hand(self.deck.deal_card(), 0);
            }
        }

        let mut out = 0;
        loop {
            if out == 12 {
                break;
            };
            out += 1;

            self.printer
                .print_message(Message::PlayerStatus, Some(&self.dealer));

            // PLAYERS ACTIONS
            for p in self.players.iter_mut().filter(|p| p.still_playing()) {
                for current_hand in 0..p.hands.len() {
                    p.play(current_hand, &mut self.deck, &self.printer)?;
                }
            }

            // DEALERS ACTIONS
            if self.dealer.still_playing() {
                println!("The Dealer will play now");
                self.dealer.play(0, &mut self.deck, &self.printer)?;
            }

            if self.should_break_loop() {
                break;
            }
        }
        self.verify_results();
        Ok(())
    }

    fn should_break_loop(&self) -> bool {
        let players = self.players.iter().all(|p| !p.still_playing());
        let dealer = !self.dealer.still_playing();
        players && dealer
    }

    fn verify_results(&mut self) {
        self.printer
            .print_message(Message::Generic(Generic::VerifyResults), None);

        let dealer_hand = self.dealer.check_hand_condition(0);
        for p in self.players.iter_mut() {
            let single_hand = p.hands.len() == 1;
            for (i, h) in p.hands.iter().enumerate() {
                let player_hand = h.check_hand();
                if single_hand {
                    println!("Comparing {}'s hand with the Dealers'", p.name);
                } else {
                    println!("Comparing {}'s {} hand with the Dealers'", p.name, i + 1);
                }
                match (&dealer_hand, &player_hand) {
                    (HandCondition::Under, HandCondition::Under) => {
                        let dealer_hand = &self.dealer.hands[0];
                        if dealer_hand > h {
                            println!("Dealer won");
                        } else if dealer_hand == h {
                            println!("Tie!");
                            p.pot += h.current_bet();
                        } else {
                            println!("Player won");
                            p.pot += 2 * h.current_bet();
                        }
                    }
                    (HandCondition::Blackjack, HandCondition::Blackjack) => {
                        println!("Tie!");
                        p.pot += h.current_bet();
                    }
                    (HandCondition::Busted, _) => {
                        println!("Player won");
                        p.pot += 2 * h.current_bet();
                    }
                    (HandCondition::Blackjack, _) => {
                        println!("Dealer won");
                    }
                    (_, HandCondition::Blackjack) => {
                        println!("Player won");
                        p.pot += (2.5 * h.current_bet() as f32).ceil() as u32;
                    }
                    (_, HandCondition::Busted) => {
                        println!("Dealer won");
                    }
                };
            }
        }
    }
}
