use std::{process::exit, io::stdin};

use crate::{game::Game, ending::{losing, winning}};

pub fn inputing(game: &mut Game) {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Couldn't read input");

    match input.trim() {
        x if x == "a" => {
            game.deal(1, true);

            println!("What NOW?");
            inputing(game);
        }

        x if x == "d" => {
            println!("{:#?}", game.debug());
        }

        x if x == "q" => {
            exit(0);
        }

        x if x == "h" => {
            println!("HELP:
            - a => deal another card,
            - q => exit the game,
            - f => final step, cards are revealed,
            \n");

            println!("What NOW?");
            inputing(game);
        }

        x if x == "f" => {
            let values = game.get_value();

            if values.1 > 21 || values.0 > values.1 {
                losing(game.clone(), values);
            }

            if values.1 > values.0 && values.1 <= 21 {
                winning(game.clone(), values);
            }
        }

        _ => {
            println!("What NOW?");
            inputing(game);
        }
    }
}