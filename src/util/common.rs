//! Boilerplate for common actions like setting up a camera or loading a sprite sheet
use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{
        camera::{Camera, Projection},
        formats::texture::ImageFormat,
        sprite::{SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle},
        Texture,
    },
};

const ARENA_HEIGHT: f32 = 500.0;
const ARENA_WIDTH: f32 = 500.0;

/// Add a camera to the world
pub fn add_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 1.0);
    world
        .create_entity()
        // A default camera can be created with standard_2d, but we instead create a camera
        // which is centered on our gameplay area (ARENA)
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            0.0,
            ARENA_HEIGHT,
            0.1,
            2000.0,
        )))
        .with(transform)
        .build();
}

/// Return a handle to a sprite sheet
pub fn load_sprites(world: &mut World, path: &str) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(path, ImageFormat::default(), (), &texture_storage)
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        String::from(path) + ".ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}
