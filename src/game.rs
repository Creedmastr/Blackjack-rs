use std::{vec, io::stdin, process::exit};

use crate::{
    cards::Card,
    ending::{losing, winning},
};
use rand::{self, Rng};

#[derive(Clone, Default, Debug)]
pub struct Game {
    pub dealer_cards: Vec<Card>,
    pub player_cards: Vec<Card>,
    pub card_list: Vec<Card>,
}

impl Game {
    pub fn init() -> Self {
        let mut result = Game::default();

        result.card_list = vec![
            Card {
                name: &"ACE",
                value: 1,
            },
            Card {
                name: "TWO",
                value: 2,
            },
            Card {
                name: "THREE",
                value: 3,
            },
            Card {
                name: "FOUR",
                value: 4,
            },
            Card {
                name: "FIVE",
                value: 5,
            },
            Card {
                name: "SIX",
                value: 6,
            },
            Card {
                name: "SEVEN",
                value: 7,
            },
            Card {
                name: "EIGHT",
                value: 8,
            },
            Card {
                name: "NINE",
                value: 9,
            },
            Card {
                name: "TEN",
                value: 10,
            },
            Card {
                name: "JACK",
                value: 11,
            },
            Card {
                name: "QUEEN",
                value: 12,
            },
            Card {
                name: "KING",
                value: 13,
            },
        ];

        result.deal(2, true);
        result.deal(2, false);

        return result;
    }

    pub fn deal(&mut self, nb_cards: u8, is_player: bool) {
        let mut random = rand::thread_rng();

        for i in 0..nb_cards {
            if is_player {
                self.player_cards
                    .push(self.card_list[random.gen_range(0..self.card_list.len())]);
            } else {
                self.dealer_cards
                    .push(self.card_list[random.gen_range(0..self.card_list.len())]);
            }
        }
    }

    pub fn get_value(&self) -> (u8, u8) {
        let mut result = (0, 0);

        for card in self.dealer_cards.clone() {
            result.0 += card.value;
        }

        for card in self.player_cards.clone() {
            result.1 += card.value
        }

        return result;
    }

    pub fn debug(&self) -> Self {
        let result = Game {
            dealer_cards: self.dealer_cards.clone(),
            player_cards: self.player_cards.clone(),
            card_list: vec![],
        };

        return result;
    }

    pub fn format_cards(&self, is_player: bool) -> String {
        let mut result = String::new();

        if !is_player {
            for i in self.dealer_cards.clone() {
                result.push_str(&i.name.to_string());
                result.push_str(", ");
            }
        } else {
            for i in self.player_cards.clone() {
                result.push_str(&i.name.to_string());
                result.push_str(", ");
            }
        }

        return result;
    }

    pub fn check(&self) {
        let values = self.get_value();

        if values.1 > 21 {
            losing(self.clone(), values);
        }
    }

    pub fn show_cards(&self) {
        let value = self.get_value();
        println!("Your CARDS are: {:#?}, and VALUE at: {}", self.format_cards(true), value.1);
        println!("The DEALER first card is: {:#?}, and VALUE at: {}", self.dealer_cards[0].name, value.0);
    }

    pub fn inputing(&mut self) {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Couldn't read input");
    
        match input.trim() {
            x if x == "a" => {
                println!("Dealing ONE card
                ");
                self.deal(1, true);

                self.show_cards();
                print!("\n");

                self.check();

                println!("What NOW?");
                self.inputing();
            }
    
            x if x == "d" => {
                println!("{:#?}", self.debug());
            }
    
            x if x == "q" => {
                exit(0);
            }
    
            x if x == "h" => {
                println!("HELP:
                - a => deal another card,
                - q => exit the game,
                - f => final step, cards are revealed,
                ");
    
                println!("What NOW?");
                self.inputing();
            }
    
            x if x == "f" => {
                let values = self.get_value();
    
                if values.1 > 21 || values.0 > values.1 {
                    losing(self.clone(), values);
                }
    
                if values.1 > values.0 && values.1 <= 21 {
                    winning(self.clone(), values);
                }
            }
    
            _ => {
                println!("What NOW?");
                self.inputing();
            }
        }
    }
}
