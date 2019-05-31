//! Boilerplate for common actions like setting up a camera or loading a sprite sheet
use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::math::{Vector2, Vector3},
    core::transform::Transform,
    core::Float,
    ecs::{prelude::*, Read, Write},
    prelude::*,
    renderer::{
        camera::{Camera, Projection},
        formats::texture::ImageFormat,
        sprite::{
            Sprite, SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle,
            TextureCoordinates,
        },
        Texture,
    },
};
use std::fs::File;
use std::path::Path;
use tiled::{Map, TmxFormat};

const ARENA_HEIGHT: f32 = 500.0;
const ARENA_WIDTH: f32 = 500.0;

/// Add a camera to the world
pub fn add_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(-(ARENA_WIDTH / 2.), ARENA_HEIGHT / 2., 1.);
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

pub fn load_map(world: &mut World, path: &str) -> Handle<Map> {
    let loader = &world.read_resource::<Loader>();
    loader.load(
        path,
        TmxFormat,
        (),
        &world.read_resource::<AssetStorage<Map>>(),
    )
}
