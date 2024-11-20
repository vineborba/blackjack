mod action;
mod card;
mod console_printer;
mod deck;
mod game;
mod hand;
mod player;
mod printer;

use console_printer::ConsolePrinter;
use game::Game;

fn main() {
    let mut game = Game::new(1, ConsolePrinter::new(1));
    game.start(1);
}
