//! Contains commonly used data structures
use amethyst::{
    input::{BindingTypes, InputEvent},
    assets::{Handle, Asset, ProcessingState},
    error::Error,
    ecs::VecStorage,
};
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

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CameraConfig {
    pub origin: (usize, usize),
    pub size: (usize, usize),
    pub znear: f32,
    pub zfar: f32,
}

impl Asset for CameraConfig {
    const NAME: &'static str = "j::CameraConfig";
    type Data = Self;
    type HandleStorage = VecStorage<Handle<CameraConfig>>;
}

impl From<CameraConfig> for Result<ProcessingState<CameraConfig>, Error> {
    fn from(camera_config: CameraConfig)
        -> Result<ProcessingState<CameraConfig>, Error> {
            Ok(ProcessingState::Loaded(camera_config))
        }
}
