//! Component for+ axis-moving players and AI

use amethyst::{
    core::{
        math::Unit,
        math::{Vector2, Vector3},
        Float, Transform,
    },
    ecs::{Component, DenseVecStorage, Entity},
    prelude::*,
};

use crate::util::data::Direction;
use std::collections::VecDeque;

pub struct Movement {
    /// Milliseconds taken to move one block
    pub speed: f32,
    /// Pixel cube size of grid when moving one step
    pub size: Float,
    /// If the movement is in the middle of interpolating to another tile, mark as busy. This means
    /// that the current action will complete before any other movements are made.
    pub busy: bool,
    /// To save us from calculating every frame, this field is calculated when we get a new
    /// direction when the movement is not busy. This is the relative vector we move towards.
    /// Once the position is reached, we mark the movement as not busy. If there is no direction
    /// (AKA, the movement should stand still) a value of (0,0,0) is supplied.
    pub target: Vector3<Float>,
}

impl Component for Movement {
    type Storage = DenseVecStorage<Self>;
}

impl Movement {
    /// Create a new movement component
    ///
    /// * `translation` - the initial starting point (same as the transform) for this component.
    /// This is useful so we can position objects on an absolute grid and later compare between them
    pub fn new(translation: Vector3<Float>, size: Float) -> Self {
        Self {
            speed: 1000f32,
            size,
            busy: false,
            target: Vector3::zeros(),
        }
    }
    pub fn set_speed(&mut self, speed: f32) -> &Self {
        self.speed = speed;
        self
    }
    /// Set the grid-aligned target using a unit vector. Under the hood, we take this unit vector and create
    /// a target coordinate to lerp to.
    pub fn set_target(&mut self, direction: Unit<Vector3<Float>>) -> &Self {
        // calculate the relative movement vector based on our supplied direction
        self.target = direction.into_inner() * self.size;
        self
    }

    pub fn set_size(&mut self, size: Float) -> &Self {
        self.size = size;
        self
    }
}
