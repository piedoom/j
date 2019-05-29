//! Boilerplate for common actions like setting up a camera or loading a sprite sheet
use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::math::{Vector2, Vector3},
    core::transform::Transform,
    core::Float,
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
    window::ScreenDimensions,
};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use tiled::parse;

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
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    loader.load(path, ImageFormat::default(), (), &texture_storage)
}

struct TileData {
    tile_size: Vector2<i32>,
    tileset_size: Vector2<i32>,
}

struct TileGrid<T> {
    columns: T,
    rows: T,
}

impl TileData {
    pub fn get_tileset_sprite_grid(&self) -> TileGrid<i32> {
        TileGrid::<i32> {
            columns: (self.tileset_size.x / self.tile_size.x) as i32,
            rows: (self.tileset_size.y / self.tile_size.y) as i32,
        }
    }
    pub fn get_tileset_offset_grid(&self) -> TileGrid<f32> {
        let grid = self.get_tileset_sprite_grid();
        TileGrid::<f32> {
            columns: 1.0 / grid.columns as f32,
            rows: 1.0 / grid.rows as f32,
        }
    }
}

/// Build a tilemap level from a tiled file.
///
/// * `world` - The current world in which to build
/// * `texture_path` - The path including extension of the tilemap image
/// * `map_path` - The path including extension to the Tiled file
///
/// Adapted from https://github.com/Temeez/Tiled-Amethyst-Example/blob/master/src/main.rs
pub fn load_and_create_tilemap(world: &mut World, texture_path: &str, map_path: &str) {
    // Load from a tilemap file
    // Load the tilemap texture. For now, we can only use one spritesheet
    let texture_handle = load_texture(world, texture_path);

    // Get the game window screen height
    let screen_height = world.read_resource::<ScreenDimensions>().height();

    // Load the tiled map
    let file = File::open(&Path::new(&format!("resources/{}", map_path))).unwrap();
    let reader = BufReader::new(file);
    let map = parse(reader).unwrap();

    // Only do the following if a layer with an ID of 1 exists. Otherwise, the map is empty!
    if let Some(m) = map.get_tileset_by_gid(1) {
        let tile_data = TileData {
            tile_size: Vector2::new(m.tile_width as i32, m.tile_height as i32),
            tileset_size: Vector2::new(m.images[0].width.clone(), m.images[0].height.clone()),
        };

        // A place to store the tile sprites in. This is useful for animated tile sprites (which
        // we are not using at the moment)
        let mut tile_sprites: Vec<Sprite> = Vec::new();

        // The x-axis needs to be reversed for TextureCoordinates
        for x in 0..tile_data.get_tileset_sprite_grid().rows {
            for y in 0..tile_data.get_tileset_sprite_grid().columns {
                // Coordinates of the 64x64 tile sprite inside the whole
                let offset = tile_data.get_tileset_offset_grid();
                let tex_coords = TextureCoordinates {
                    left: y as f32 * offset.columns,
                    right: (y + 1) as f32 * offset.columns,
                    top: x as f32 * offset.rows,
                    bottom: (x + 1) as f32 * offset.rows,
                };

                let sprite = Sprite {
                    width: tile_data.tile_size.x as f32,
                    height: tile_data.tile_size.y as f32,
                    offsets: [0.0, 0.0],
                    tex_coords,
                };

                tile_sprites.push(sprite);
            }
        }

        // A sheet of sprites.. so all the tile sprites
        let sprite_sheet = SpriteSheet {
            texture: texture_handle,
            sprites: tile_sprites,
        };

        // Insert the sprite sheet, which consists of all the tile sprites,
        // into world resources for later use
        let sprite_sheet_handle = {
            let loader = world.read_resource::<Loader>();
            let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
            loader.load_from_data(sprite_sheet, (), &sprite_sheet_storage)
        };

        // Now that all the tile sprites/textures are loaded in
        // we can start drawing the tiles for our viewing pleasure

        // Get the base layer of the map
        let layer: &tiled::Layer = &map.layers[0];

        // Loop the row first and then the individual tiles on that row
        // and then switch to the next row
        // i_row = row iteration number
        // i_column = column iteration number
        for (i_row, row) in layer.tiles.iter().enumerate().clone() {
            for (i_column, &tile) in row.iter().enumerate() {
                // Do nothing with empty tiles
                if tile == 0 {
                    continue;
                }

                // Tile ids start from 1 but tileset sprites start from 0
                let tile = tile - 1;

                // Renderer for the tile
                let tile_renderer = SpriteRender {
                    sprite_sheet: sprite_sheet_handle.clone(),
                    sprite_number: tile as usize,
                };

                // Where we should draw the tile?
                let tile_transform = {
                    // Bottom Left is 0,0 so we flip it to Top Left with the
                    // ScreenDimensions.height since tiled coordinates start from top
                    let coordinates = (
                        Float::from(i_column as f32 * tile_data.tile_size.x as f32),
                        Float::from(
                            1f32 - (i_row as f32 * tile_data.tile_size.y as f32),
                        ),
                        Float::from(-10.0),
                    );
                    // Offset the positions by half the tile size so they're nice and snuggly on the screen
                    // Alternatively could use the Sprite offsets instead: [-32.0, 32.0]. Depends on the use case I guess.
                    let offset = (
                        Float::from(tile_data.tile_size.x as f32 / 2.0),
                        Float::from(-tile_data.tile_size.y as f32 / 2.0),
                    );
                    Transform::from(Vector3::new(
                        offset.0 + coordinates.0,
                        offset.1 + coordinates.1,
                        coordinates.2,
                    ))
                };
                // Create the tile entity
                world
                    .create_entity()
                    .with(tile_transform)
                    .with(tile_renderer)
                    .build();
            }
        }
    }
}
