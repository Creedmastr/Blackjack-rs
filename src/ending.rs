use std::{process::exit, io::stdout};

use crate::{game::Game, terminal, main_loop};

fn restart() {
    println!("Do YOU want to RESTART (y/n)?");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Couldn't read input");
    match input.trim() {
        "y" => main_loop(),

        _ => exit(0)
    }
}

pub fn losing(game: &mut Game, values: (u8, u8)) {
    terminal::clear();
    game.chips -= game.betting;

    println!("You have LOST this round, and LOST {} chips!", game.betting);

    println!("Dealer cards: {:#?}, and their VALUE is: {:#?}", game.format_cards(false), values.0);
    println!("Your cards: {:#?}, and their VALUE is: {:#?}", game.format_cards(true), values.1);

    println!("You have now {} chips!", game.chips);



    restart();

    exit(0);
}

pub fn winning(game: &mut Game, values: (u8, u8)) {
    terminal::clear();
    game.chips += game.betting;

    println!("You have WON this round, and WON {} chips!", game.betting);

    println!("Dealer cards: {:#?}, and their VALUE is: {:#?}", game.format_cards(false), values.0);
    println!("Your cards: {:#?}, and their VALUE is: {:#?}", game.format_cards(true), values.1);

    restart();

    exit(0);
}