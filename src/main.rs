#![allow(unused_imports, unused_variables)]

use cards::Card;
use rand;
use std::{io::stdin, process::exit};

mod cards;
mod game;
mod ending;

fn main() {
    let mut game = game::Game::init();

    game.show_cards();

    game.check(); // Checks if you have already lost

    println!("\nWhat do YOU want to DO? REMEMBER: h for HELP.");

    game.inputing();
    
    println!("{:#?}", game.debug());
}
