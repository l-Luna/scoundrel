use std::collections::VecDeque;
use macroquad::prelude::{Color, RED, BLACK};
use rand::rng;
use rand::seq::SliceRandom;

use Card::*;

#[derive(Copy, Clone)]
pub enum Card{
    Weapon(u8),
    Health(u8),
    Monster(bool, u8)
}

impl Card{
    pub fn describe(&self) -> String{
        match self{
            Weapon(strength) => format!("{} of Diamonds", Self::describe_n(*strength)),
            Health(strength) => format!("{} of Hearts", Self::describe_n(*strength)),
            Monster(ty, strength) =>
                format!("{} of {}", Self::describe_n(*strength), if *ty { "Spades" } else { "Clubs" }),
        }
    }
    
    pub fn describe_n(n: u8) -> String{
        match n{
            1 => "Ace".to_owned(),
            11 => "Jack".to_owned(),
            12 => "Queen".to_owned(),
            13 => "King".to_owned(),
            14 => "Ace".to_owned(),
            n => n.to_string()
        }
    }
    
    pub fn colour(&self) -> Color{
        match self{
            Weapon(..) | Health(..) => RED,
            Monster(..) => BLACK
        }
    }
}

pub type Deck = VecDeque<Card>;

pub fn gen_deck() -> Deck{
    let mut deck = Deck::new();
    for i in 2..10 {
        deck.push_back(Card::Weapon(i));
        deck.push_back(Card::Health(i));
    }
    for i in 2..14 {
        deck.push_back(Card::Monster(true, i));
        deck.push_back(Card::Monster(false, i));
    }
    deck.make_contiguous().shuffle(&mut rng());
    deck
}