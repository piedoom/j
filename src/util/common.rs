//! Boilerplate for common actions like setting up a camera or loading a sprite sheet
use amethyst::{
    assets::{AssetStorage, Handle, Loader, RonFormat},
    core::transform::Transform,
    ecs::prelude::*,
    renderer::{
        camera::{Camera, Projection},
        formats::texture::ImageFormat,
        sprite::{
            SpriteSheet, SpriteSheetFormat, SpriteSheetHandle,
        },
        Texture,
    },
};




const ARENA_HEIGHT: f32 = 500.0;
const ARENA_WIDTH: f32 = 500.0;

/// Return a handle to a sprite sheet
pub fn load_sprites(world: &mut World, path: &str) -> SpriteSheetHandle {
    let texture_handle = load_texture(world, path);
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        String::from(path) + ".ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

pub fn load_texture(world: &mut World, path: &str) -> Handle<Texture> {
    let loader = world.read_resource::<Loader>();
    loader.load(
        path,
        ImageFormat::default(),
        (),
        &world.read_resource::<AssetStorage<Texture>>(),
    )
}
