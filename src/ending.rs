use std::process::exit;

use crate::game::Game;

pub fn losing(game: Game, values: (u8, u8)) {
    println!("You have LOST!");

    println!("Dealer cards' VALUE: {:#?}", values.0);
    println!("Your cards' VALUE: {:#?}", values.1);

    exit(0);
}

pub fn winning(game: Game, values: (u8, u8)) {
    println!("You have WON!");

    println!("Dealer cards' VALUE: {:#?}", values.0);
    println!("Your cards' VALUE: {:#?}", values.1);

    exit(0);
}