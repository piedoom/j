use crate::{
    components::{Movement, Player},
    util::data::{Action, ActionEvent},
};
use amethyst::core::{
    math::{Unit, Vector3},
    EventReader, Float,
};
use amethyst::core::{Time, Transform};
use amethyst::ecs::{
    DenseVecStorage, Join, Read, ReadStorage, ReaderId, Resources, System, SystemData, Write,
    WriteStorage,
};
use amethyst::input::InputEvent;
use amethyst::shrev::EventChannel;

pub struct PlayerSystem {
    reader: Option<ReaderId<ActionEvent>>,
    /// Unit vector that keeps track of keyboard movements
    direction: Unit<Vector3<Float>>,
}

impl Default for PlayerSystem {
    fn default() -> Self {
        Self {
            reader: None,
            direction: Unit::new_unchecked(Vector3::zeros()),
        }
    }
}

impl<'a> System<'a> for PlayerSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, Transform>,
        WriteStorage<'a, Movement>,
        Write<'a, EventChannel<ActionEvent>>,
        Read<'a, Time>,
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.reader = Some(
            res.fetch_mut::<EventChannel<ActionEvent>>()
                .register_reader(),
        );
    }

    fn run(&mut self, (players, transforms, mut movements, events, time): Self::SystemData) {
        // add respective data to our movement/direction so our movement system can handle it properly

        for event in events.read(self.reader.as_mut().unwrap()) {
            match event {
                InputEvent::ActionReleased(action) => match action {
                    Action::Left => self.direction.as_mut_unchecked().x = Float::from(0f32),
                    Action::Right => self.direction.as_mut_unchecked().x = Float::from(0f32),
                    Action::Up => self.direction.as_mut_unchecked().y = Float::from(0f32),
                    Action::Down => self.direction.as_mut_unchecked().y = Float::from(0f32),
                    _ => (),
                },
                InputEvent::ActionPressed(action) => match action {
                    Action::Left => self.direction.as_mut_unchecked().x = Float::from(-1f32),
                    Action::Right => self.direction.as_mut_unchecked().x = Float::from(1f32),
                    Action::Up => self.direction.as_mut_unchecked().y = Float::from(1f32),
                    Action::Down => self.direction.as_mut_unchecked().y = Float::from(-1f32),
                    _ => (),
                },

                _ => (),
            }
        }

        for (player, movement) in (&players, &mut movements).join() {
            movement.set_direction(self.direction);
        }
    }
}
