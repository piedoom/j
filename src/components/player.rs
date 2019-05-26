use amethyst::{
    core::{Transform, math::Vector3, math::Unit, Float},
    ecs::{Component, DenseVecStorage, Entity},
    prelude::*,
};

#[derive(Default)]
pub struct Player {}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}