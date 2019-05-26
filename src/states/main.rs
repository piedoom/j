use amethyst::{
    prelude::*,
    core::Transform,
    renderer::SpriteRender,
};
use crate::{
    components::*,
    util,
};
pub struct MainGameState;

impl SimpleState for MainGameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        util::add_camera(world);
        let character_sheet = util::load_sprites(world, "textures/chars.png");

        // assemble an entity
        world.create_entity()
            .with(Transform::default())
            .with(SpriteRender {
                sprite_sheet: character_sheet.clone(),
                sprite_number: 0,
            })
            .with(Movement::default())
            .with(Player::default())
            .build();
    }
}