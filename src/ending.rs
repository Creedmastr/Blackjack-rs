use std::{process::exit, io::stdout};

use crate::{game::Game, terminal, main_loop};

fn restart(game: &mut Game) {
    println!("Do YOU want to RESTART (y/n)?");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Couldn't read input");
    match input.trim() {
        "y" => main_loop(game),

        _ => exit(0)
    }
}

pub fn losing(game: &mut Game, values: (i8, i8)) {
    terminal::clear();
    game.chips -= game.betting;

    println!("You have LOST this round, and LOST {} chips!", game.betting);

    println!("Dealer cards: {:#?}, and their VALUE is: {:#?}", game.format_cards(false), values.0);
    println!("Your cards: {:#?}, and their VALUE is: {:#?}", game.format_cards(true), values.1);

    if game.chips <= 0 {
        println!("You DON'T have ANY chips LEFT. \n So, the game is CONCLUDING");
        exit(0)
    }

    println!("You have now {} chips!", game.chips);

    restart(game);
}

pub fn winning(game: &mut Game, values: (i8, i8)) {
    terminal::clear();
    game.chips += game.betting;

    println!("You have WON this round, and WON {} chips!", game.betting);

    println!("Dealer cards: {:#?}, and their VALUE is: {:#?}", game.format_cards(false), values.0);
    println!("Your cards: {:#?}, and their VALUE is: {:#?}", game.format_cards(true), values.1);

    restart(game);
}