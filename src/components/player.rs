use amethyst::{
    core::{math::Unit, math::Vector3, Float, Transform},
    ecs::{Component, DenseVecStorage, Entity},
    prelude::*,
};

#[derive(Default)]
pub struct Player {}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}
