#![allow(unused_imports, unused_variables)]

use cards::Card;
use game::Game;
use rand;
use std::{io::stdin, process::exit};

mod cards;
mod game;
mod ending;
mod terminal;

fn main_loop(game: &mut Game) {
    game.init();

    game.show_cards();
    
    game.check(); // Checks if you have already lost

    println!("\nWhat do YOU want to DO? REMEMBER: h for HELP.");

    game.menu();
}

fn main() {
    let mut game = game::Game::default();

    main_loop(&mut game);
}
