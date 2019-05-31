use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Default)]
pub struct Player {}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}
