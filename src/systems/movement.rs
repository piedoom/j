use crate::components::Movement;
use amethyst::core::Time;
use amethyst::core::{math::Unit, math::Vector3, Float, Transform};
use amethyst::ecs::{prelude::*, Join, Read, ReadStorage, Resources, System, WriteStorage};
use amethyst::prelude::*;

pub struct MovementSystem {}

impl Default for MovementSystem {
    fn default() -> Self {
        Self {}
    }
}

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        WriteStorage<'a, Movement>,
        WriteStorage<'a, Transform>,
        Read<'a, Time>,
    );

    fn run(&mut self, (mut movements, mut transforms, time): Self::SystemData) {
        for (mut movement, transform) in (&mut movements, &mut transforms).join() {

            // First, check if we're already busy.
            // If we aren't busy, we need to check if our target is non-zero, which means movement
            // is required.
            if movement.busy && movement.target == Vector3::zeros() { return; }

            // Now that we know we are free to move, lerp towards our relative target
            transform.append_translation(movement.target);
        }
    }
}
