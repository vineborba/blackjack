use crate::{
    deck::Deck,
    player::{Player, PlayerKind, PlayerStatus},
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
        'round: loop {
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
            let mut must_remove = vec![];
            for p in self.players.iter_mut() {
                for current_hand in 0..p.hands.len() {
                    match p.play(current_hand, &mut self.deck)? {
                        PlayerStatus::Playing => (),
                        PlayerStatus::Won => {
                            println!("{} has won this round against the Dealer!", p.name);
                        }
                        PlayerStatus::Standing | PlayerStatus::Lost => {
                            must_remove.push(p.name.clone());
                        }
                    }
                }
            }
            self.players.retain(|p| !must_remove.contains(&p.name));

            // DEALERS ACTIONS
            if self.dealer.still_playing() {
                println!("The Dealer will play now");
                match self.dealer.play(0, &mut self.deck)? {
                    PlayerStatus::Won => {
                        println!("The Dealer has won! Better luck next time!");
                        break 'round;
                    }
                    PlayerStatus::Lost => {
                        println!("The Dealer lost! All players won this round!");
                        break 'round;
                    }
                    _ => (),
                };
            }

            if self.should_break_loop() {
                break;
            }
        }
        Ok(())
    }

    fn should_break_loop(&self) -> bool {
        let players = self.players.iter().all(|p| !p.still_playing());
        let dealer = !self.dealer.still_playing();
        players && dealer
    }
}
