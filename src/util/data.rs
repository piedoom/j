//! Contains commonly used data structures
use amethyst::input::{BindingTypes, InputEvent};
use serde::{Deserialize, Serialize};
use std::fmt;

/// All available action keys in the game
#[derive(Debug, Hash, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
}

/// All cardinal and diagonal directions
#[derive(Debug, Hash, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:?}", *self))
    }
}

pub struct GameBindings;
impl BindingTypes for GameBindings {
    type Axis = String;
    type Action = Action;
}

pub type ActionEvent = InputEvent<Action>;
