#![allow(unused_imports, unused_variables)]

use cards::Card;
use rand;
use std::{io::stdin, process::exit};

use crate::looping::inputing;

mod cards;
mod game;
mod looping;
mod ending;

fn main() {
    let mut game = game::Game::init();

    println!("Your CARDS are: {:#?}", game.format_cards(true));
    println!("The DEALER first card is: {:#?}", game.dealer_cards[0].name);

    println!("What do YOU want to DO? REMEMBER: h for HELP.");

    inputing(&mut game);
    
    println!("{:#?}", game.debug());
}
