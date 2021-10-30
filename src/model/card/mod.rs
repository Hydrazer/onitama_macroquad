use strum_macros::EnumIter;

use crate::model::card::Card::*;
use crate::model::piece;

#[derive(Clone, Debug)]
pub enum CardItem {
  Empty,
  Middle,
  Goto,
}

#[derive(Debug, EnumIter, PartialEq, Eq)]
pub enum Card {
  Bear,
  Boar,
  Cobra,
  Crab,
  Crane,
  Dog,
  Dragon,
  Eel,
  Elephant,
  Fox,
  Frog,
  Giraffe,
  Goose,
  Horse,
  Igauna,
  Kirin,
  Mantis,
  Monkey,
  Mouse,
  Otter,
  Ox,
  Panda,
  Phoenix,
  Rabbit,
  Rat,
  Rooster,
  Sable,
  SeaSnake,
  Tanuki,
  Tiger,
  Turtle,
  Viper,
}

impl Card {
  pub fn value(&self) -> Vec<Vec<CardItem>> {
    let card_ref = match self {
      Bear => [".....", ".oo..", "..O..", "...o.", "....."],
      Boar => [".....", "..o..", ".oOo.", ".....", "....."],
      Cobra => [".....", "...o.", ".oO..", "...o.", "....."],
      Crab => [".....", "..o..", "o.O.o", ".....", "....."],
      Crane => [".....", "..o..", "..O..", ".o.o.", "....."],
      Dog => [".....", ".o...", ".oO..", ".o...", "....."],
      Dragon => [".....", "o...o", "..O..", ".o.o.", "....."],
      Eel => [".....", ".o...", "..O.o", ".o...", "....."],
      Elephant => [".....", ".o.o.", ".oOo.", ".....", "....."],
      Fox => [".....", "...o.", "..Oo.", "...o.", "....."],
      Frog => [".....", ".o...", "o.O..", "...o.", "....."],
      Giraffe => [".....", "o...o", "..O..", "..o..", "....."],
      Goose => [".....", ".o...", ".oOo.", "...o.", "....."],
      Horse => [".....", "..o..", ".oO..", "..o..", "....."],
      Igauna => [".....", "o.o..", "..O..", "...o.", "....."],
      Kirin => [".o.o.", ".....", "..O..", ".....", "..o.."],
      Mantis => [".....", ".o.o.", "..O..", "..o..", "....."],
      Monkey => [".....", ".o.o.", "..O..", ".o.o.", "....."],
      Mouse => [".....", "..o..", "..Oo.", ".o...", "....."],
      Otter => [".....", ".o...", "..O.o", "....o.", "....."],
      Ox => [".....", "..o..", "..Oo.", "..o..", "....."],
      Panda => [".....", "..oo.", "..O..", ".o...", "....."],
      Phoenix => [".....", ".o.o.", "o.O.o", ".....", "....."],
      Rabbit => [".....", "...o.", "..O.o", ".o...", "....."],
      Rat => [".....", "..o..", ".oO..", "...o.", "....."],
      Rooster => [".....", "...o.", ".oOo.", ".o...", "....."],
      Sable => [".....", "...o.", "o.O..", ".o...", "....."],
      SeaSnake => [".....", "..o..", "..O.o", ".o...", "....."],
      Tanuki => [".....", "..o.o", "..O..", ".o...", "....."],
      Tiger => ["..o..", ".....", "..O..", "..o..", "....."],
      Turtle => [".....", ".....", "o.O.o", ".o.o.", "....."],
      Viper => [".....", "..o..", "o.O..", "...o.", "....."],
    };

    let mut card: Vec<Vec<CardItem>> = vec![vec![]; 5];

    for (i, line) in (0..).zip(card_ref.iter()) {
      for (_, item) in (0..).zip(line.chars()) {
        card[i].push(match item {
          'o' => CardItem::Goto,
          'O' => CardItem::Middle,
          _ => CardItem::Empty,
        })
      }
    }

    card
  }

  pub fn colour(&self) -> piece::Colour {
    match self {
      Bear => piece::Colour::Blue,
      Boar => piece::Colour::Red,
      Cobra => piece::Colour::Red,
      Crab => piece::Colour::Blue,
      Crane => piece::Colour::Blue,
      Dog => piece::Colour::Blue,
      Dragon => piece::Colour::Red,
      Eel => piece::Colour::Blue,
      Elephant => piece::Colour::Red,
      Fox => piece::Colour::Red,
      Frog => piece::Colour::Red,
      Giraffe => piece::Colour::Blue,
      Goose => piece::Colour::Blue,
      Horse => piece::Colour::Red,
      Igauna => piece::Colour::Red,
      Kirin => piece::Colour::Red,
      Mantis => piece::Colour::Red,
      Monkey => piece::Colour::Blue,
      Mouse => piece::Colour::Blue,
      Otter => piece::Colour::Red,
      Ox => piece::Colour::Blue,
      Panda => piece::Colour::Red,
      Phoenix => piece::Colour::Blue,
      Rabbit => piece::Colour::Blue,
      Rat => piece::Colour::Red,
      Rooster => piece::Colour::Red,
      Sable => piece::Colour::Blue,
      SeaSnake => piece::Colour::Blue,
      Tanuki => piece::Colour::Blue,
      Tiger => piece::Colour::Blue,
      Turtle => piece::Colour::Red,
      Viper => piece::Colour::Red,
    }
  }
}
