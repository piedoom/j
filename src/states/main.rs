use crate::{components::*, util};
use amethyst::{
    core::{math::Vector3, Float, Transform},
    prelude::*,
    renderer::SpriteRender,
};
pub struct MainGameState;

impl SimpleState for MainGameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        util::add_camera(world);
        let character_sheet = util::load_sprites(world, "textures/chars.png");

        // assemble an entity
        let transform = Transform::default();
        world
            .create_entity()
            .with(transform.clone())
            .with(SpriteRender {
                sprite_sheet: character_sheet.clone(),
                sprite_number: 0,
            })
            .with(Movement::default())
            .with(Player::default())
            .build();
    }
}
