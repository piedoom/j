//! Component for+ axis-moving players and AI

use amethyst::{
    core::{Transform, math::Vector3, math::Unit, Float},
    ecs::{Component, DenseVecStorage, Entity},
    prelude::*,
};

pub struct Movement {
    /// Blocks moved per second
    pub speed: f32,
    /// Scalar value for movement
    pub direction: Unit<Vector3<Float>>,
}

impl Default for Movement {
    fn default() -> Self {
        Self {
            speed: 1f32,
            direction: Unit::new_unchecked(Vector3::new(Float::from(0f32), Float::from(0f32), Float::from(0f32)))
        }
    }
}

impl Component for Movement {
    type Storage = DenseVecStorage<Self>;
}

impl Movement {
    pub fn speed(&mut self, speed: f32) -> &Self {
        self.speed = speed;
        self
    }
    pub fn direction(&mut self, direction: (f32, f32)) -> &Self {
        self.direction = Unit::new_unchecked(Vector3::new(Float::from(direction.0), Float::from(direction.1), Float::from(0f32)));
        self
    }
}