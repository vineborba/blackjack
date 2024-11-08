mod action;
mod card;
mod deck;
mod game;
mod hand;
mod player;

use game::Game;

fn main() {
    let mut game = Game::new(1);
    game.start(1);
    println!("Exiting!");
}
