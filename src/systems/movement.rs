use crate::components::{GridMovement, Movement};
use amethyst::animation::InterpolationFunction::Linear;
use amethyst::core::Time;
use amethyst::core::{math::Unit, math::Vector3, Float, Transform};
use amethyst::ecs::{prelude::*, Join, Read, ReadStorage, Resources, System, WriteStorage};
use amethyst::prelude::*;

pub struct GridMovementSystem {}

impl Default for GridMovementSystem {
    fn default() -> Self {
        Self {}
    }
}

impl<'a> System<'a> for GridMovementSystem {
    type SystemData = (
        WriteStorage<'a, GridMovement>,
        WriteStorage<'a, Transform>,
        Read<'a, Time>,
    );

    fn run(&mut self, (mut movements, mut transforms, time): Self::SystemData) {
        for (mut movement, transform) in (&mut movements, &mut transforms).join() {
            // Lerp from the movement start to the movement end.
            transform.set_translation(movement.start.lerp(
                &movement.target,
                Float::from(movement.normalize_duration(time.absolute_time())),
            ));

            //transform.set_translation(movement.target);
        }
    }
}

pub struct MovementSystem {}

impl Default for MovementSystem {
    fn default() -> Self {
        Self {}
    }
}

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        ReadStorage<'a, Movement>,
        WriteStorage<'a, Transform>,
        Read<'a, Time>,
    );

    fn run(&mut self, (movements, mut transforms, time): Self::SystemData) {
        for (movement, transform) in (&movements, &mut transforms).join() {
            // move at a constant speed in the direction
            transform.append_translation(movement.next(&time));
        }
    }
}
