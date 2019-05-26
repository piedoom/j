use amethyst::core::{Transform, math::Vector3, math::Unit, Float};
use amethyst::ecs::{prelude::*, Join, Read, ReadStorage, Resources, System, WriteStorage};
use amethyst::prelude::*;
use crate::components::Movement;
use amethyst::core::Time;

#[derive(Default)]
pub struct MovementSystem { }

// impl Default for MovementSystem {
//     fn default() -> Self {
//         Self { }
//     }
// }

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        ReadStorage<'a, Movement>,
        WriteStorage<'a, Transform>,
        Read<'a, Time>,
    );

    fn run(&mut self, (movements, mut transforms, time): Self::SystemData) {
        for (movement, transform) in (&movements, &mut transforms).join() {
            // Move based on vector value
            transform.prepend_translation_along(movement.direction, movement.speed);
        }
    }
}