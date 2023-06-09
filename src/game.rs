use std::vec;

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

        let values = self.get_value();

        if values.1 > 21 {
            losing(self.clone(), values);
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

        let mut intermediate: Vec<String> = vec![];

        if !is_player {
            for i in self.dealer_cards.clone() {
                intermediate.push(i.name.to_string());
            }
        } else {
            for i in self.player_cards.clone() {
                intermediate.push(i.name.to_string());
            }
        }

        match intermediate.len() {
            2 => {
                result = format!("{}, {}", intermediate[0], intermediate[1]);
            }

            3 => {
                result = format!(
                    "{}, {}, {}",
                    intermediate[0], intermediate[1], intermediate[2]
                );
            }

            4 => {
                result = format!(
                    "{}, {}, {}, {}",
                    intermediate[0], intermediate[1], intermediate[2], intermediate[3]
                );
            }

            _ => {}
        }

        return result;
    }
}
