//! Component for+ axis-moving players and AI

use amethyst::{
    core::{
        math::Unit,
        math::{Vector2, Vector3},
        timing::Time,
        Float, Transform,
    },
    ecs::{Component, DenseVecStorage, Entity},
    prelude::*,
};

use std::time::Duration;

use crate::util::data::Direction;
use std::collections::VecDeque;

pub struct GridMovement {
    /// Milliseconds taken to move to one position
    pub duration: Duration,
    /// Pixel cube size of grid when moving one step
    pub size: Float,
    /// To save us from calculating every frame, this field is calculated when we get a new
    /// direction when the movement is not busy
    pub target: Vector3<Float>,
    /// The same as target - start. A fully zero value means no movement.
    pub target_local: Vector3<Float>,
    /// We need to store the starting vector somewhere, so we can lerp
    pub start: Vector3<Float>,
    /// Holds duration of when mover started moving. Used to calculate lerp values
    pub start_time: Duration,
}

impl Component for GridMovement {
    type Storage = DenseVecStorage<Self>;
}

impl GridMovement {
    /// Create a new movement component
    ///
    /// * `translation` - the initial starting point (same as the transform) for this component.
    /// This is useful so we can position objects on an absolute grid and later compare between them
    pub fn new(translation: Vector3<Float>, size: Float) -> Self {
        Self {
            duration: Duration::from_millis(200u64),
            size,
            target: Vector3::zeros(),
            target_local: Vector3::zeros(),
            start: Vector3::zeros(),
            start_time: Duration::default(),
        }
    }
    pub fn set_duration(&mut self, duration: Duration) -> &Self {
        self.duration = duration;
        self
    }
    /// Set the grid-aligned target using a unit vector. Under the hood, we take this unit vector and create
    /// a target coordinate to lerp to.
    ///
    /// * `direction` - A `Vector3` unit pointing in a direction. In this case, it should always be
    /// integer values.
    /// * `transform` - The associated transform of this component's entity. We use it to read our
    /// current transforms so we don't need to keep a copy of the data.
    pub fn set_move(
        &mut self,
        direction: &Unit<Vector3<Float>>,
        transform: &Transform,
        time: Duration,
    ) -> &Self {
        // If our current transform position is the same as our target position, that means we are
        // free to start another movement along the grid.
        if transform.translation() == &self.target {
            // Replace our start and target positions
            self.start = transform.translation().clone();
            self.target_local = direction.into_inner() * self.size;
            self.target = self.start + self.target_local;
            // Update the time at which we started this movement
            self.start_time = time;
        }
        self
    }

    pub fn set_size(&mut self, size: Float) -> &Self {
        self.size = size;
        self
    }

    /// Return a number from 0 to 1. Used for movement interpolation
    pub fn normalize_duration(&self, current_time: Duration) -> Float {
        let mut difference = current_time - self.start_time;
        // cap value if too high. Difference should always be less than or equal to duration. If
        // not, we can just return 1.
        if !(difference <= self.duration) {
            return Float::from(1.);
        }
        // normalize and return
        Float::from(difference.div_duration_f64(self.duration))
    }
}

/// Mode in which items are snapped to a grid
///
/// * `None` - no snapping. Freely move.
/// * `Nearest` - snap to the nearest grid point, even if it is "backwards". Can cause "rubber band"
/// type look.
/// * `Continuation` - finish any movement started, even if it is not the nearest snap point
pub enum SnapMode {
    None,
    Nearest,
    Continuation,
}

/// Instead of calculating the next transform from a block over, `FreeMovement`
/// will continuously move in a direction, and then has the option to align to the nearest grid
/// while maintaining speed when the movement ends. This is preferred with players or AI that
/// move smoothly, or need to not move faster on diagonals.
pub struct Movement {
    /// Move this many pixels, times the `size` per second
    pub speed: Float,
    /// Pixel cube size of grid. This is used to calculate speed, so it must be set even if grid
    /// snapping is disabled.
    pub size: Float,
    direction: Unit<Vector3<Float>>,
    pub snap_mode: SnapMode,
}

impl Default for Movement {
    fn default() -> Self {
        Self {
            speed: Float::from(128.),
            size: Float::from(32.),
            direction: Unit::new_unchecked(Vector3::zeros()),
            snap_mode: SnapMode::None,
        }
    }
}

impl Movement {
    pub fn set_direction(&mut self, direction: Unit<Vector3<Float>>) -> &Self {
        // If direction is ever zero, we will need to call a special method to stop on the grid
        self.direction = direction;
        self
    }

    /// Get the transform to which to append to the current transform
    pub fn next(&self, time: &Time) -> Vector3<Float> {
        let scalar = (Float::from(time.delta_seconds()) * self.speed);
        self.direction.scale(scalar)
    }
}

impl Component for Movement {
    type Storage = DenseVecStorage<Self>;
}
