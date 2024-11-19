use crate::{
    deck::Deck,
    hand::HandCondition,
    player::{Player, PlayerKind},
};

#[derive(Debug)]
pub struct Game {
    number_of_rounds: u16,
    played_rounds: u16,
    players: Vec<Player>,
    dealer: Player,
    deck: Deck,
}

impl Game {
    pub fn new(_number_of_rounds: u16) -> Self {
        Self {
            number_of_rounds: 1,
            played_rounds: 0,
            players: vec![],
            deck: Deck::new(),
            dealer: Player::new("Dealer".into(), 0, PlayerKind::Dealer),
        }
    }

    pub fn start(&mut self, players_count: u8) {
        while self.played_rounds < self.number_of_rounds {
            self.setup_game(players_count);
            self.run_round().unwrap_or_else(|e| {
                panic!("Failed to run round {}: {}", self.number_of_rounds + 1, e)
            });
            self.played_rounds += 1;
        }
    }

    fn setup_game(&mut self, players_count: u8) {
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
        println!("Starting now round {}!\n\n", self.played_rounds + 1);

        println!("Shuffling deck...\n");
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

            println!("{}", "*".repeat(90));
            println!(
                "The dealer has the following hand:\n{}",
                self.dealer.hands[0]
            );
            println!("{}", "*".repeat(90));

            // PLAYERS ACTIONS
            for p in self.players.iter_mut().filter(|p| p.still_playing()) {
                for current_hand in 0..p.hands.len() {
                    p.play(current_hand, &mut self.deck)?;
                }
            }

            // DEALERS ACTIONS
            if self.dealer.still_playing() {
                println!("The Dealer will play now");
                self.dealer.play(0, &mut self.deck)?;
            }

            if self.should_break_loop() {
                break;
            }
        }
        println!("Verifying results now");
        self.verify_results();
        Ok(())
    }

    fn should_break_loop(&self) -> bool {
        let players = self.players.iter().all(|p| !p.still_playing());
        let dealer = !self.dealer.still_playing();
        players && dealer
    }

    fn verify_results(&mut self) {
        let dealer_hand = self.dealer.check_hand_condition(0);
        dbg!(&self.players);
        for p in self.players.iter_mut() {
            dbg!(2);
            let single_hand = p.hands.len() == 1;
            dbg!(3);
            for (i, h) in p.hands.iter().enumerate() {
                dbg!(4);
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
