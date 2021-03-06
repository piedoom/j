//! Boilerplate for common actions like setting up a camera or loading a sprite sheet
use crate::{
    components::*, 
    util,
    util::data::CameraConfig,
};

use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::math::{Vector2, Vector3},
    core::transform::{Transform, Parent},
    core::Float,
    ecs::{prelude::*, Read, Write},
    prelude::*,
    renderer::{
        sprite::{
            Sprite, SpriteRender, SpriteSheet, SpriteSheetHandle,
            TextureCoordinates
        },
        Texture,
        camera::{Camera, Projection},
    },
};


use tiled::{Map};

pub struct MainGameState {
    pub map_handle: Handle<Map>,
    pub texture_handle: Handle<Texture>,
    pub camera_config_handle: Handle<CameraConfig>,
    pub player_spritesheet_handle: Handle<SpriteSheet>,
}

impl SimpleState for MainGameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        // create the map and other loaded stuff
        type SystemData<'a> = (
            Entities<'a>,
            Write<'a, AssetStorage<Map>>,
            Write<'a, AssetStorage<CameraConfig>>,
            WriteStorage<'a, Player>,
            WriteStorage<'a, Movement>,
            WriteStorage<'a, Transform>,
            WriteStorage<'a, SpriteRender>,
            WriteStorage<'a, Camera>,
            WriteStorage<'a, Parent>,
            Read<'a, AssetStorage<SpriteSheet>>,
            ReadExpect<'a, Loader>,
        );

        data.world.exec(
        |(
            entities,
            map_storage,
            camera_config_storage,
            mut player_storage,
            mut movement_storage,
            mut transform_storage,
            mut sprite_render_storage,
            mut camera_storage,
            mut parent_storage,
            sprite_sheet_storage,
            loader,
        ): SystemData| {

            // Build the player
            let player = entities
                .build_entity()
                .with(Transform::default(), &mut transform_storage)
                .with(SpriteRender {
                    sprite_sheet: self.player_spritesheet_handle.clone(),
                    sprite_number: 0,
                }, &mut sprite_render_storage)
                .with(Movement::default(), &mut movement_storage)
                .with(Player::default(), &mut player_storage)
                .build();

            // Build the camera
            let camera_config = camera_config_storage.get(&self.camera_config_handle.clone()).unwrap();
            entities
                .build_entity()
                .with(Camera::from(
                    Projection::orthographic(
                        camera_config.origin.0 as f32,
                        camera_config.size.0 as f32,
                        camera_config.origin.1 as f32,
                        camera_config.size.1 as f32,
                        camera_config.znear,
                        camera_config.zfar,
                    )
                ), &mut camera_storage)
                .with(Transform::from(Vector3::new(
                     Float::from(0.0),
                     Float::from(0.0),
                     Float::from(1.0),
                )), &mut transform_storage)
                .with(Parent::new(player), &mut parent_storage)
                .build();

            // Build the map
            let map = map_storage.get(&self.map_handle.clone()).unwrap();
                // Now, we need to loop over each tileset. A tileset is - here - the same as a generated spritesheet.
                // Here, we reutrn `MapData`, which is just a struct wrapper for the tile data and spritesheet. In
                // the future, we may want a hashmap with the tileset ID
                let map_data: (Vec<(MapData)>) = map
                    .tilesets
                    .iter()
                    .map(|set| {
                        let tile_data = TileData {
                            tile_size: Vector2::new(set.tile_width as i32, set.tile_height as i32),
                            tileset_size: Vector2::new(
                                set.images[0].width.clone(),
                                set.images[0].height.clone(),
                            ),
                        };

                        // A place to store the tile sprites in. This is useful for animated tile sprites (which
                        // we are not using at the moment)
                        let mut tile_sprites: Vec<Sprite> = Vec::with_capacity(1);

                        // Map our image to a texturecoordinates, so we can load the map directly without needing
                        // a spritesheet ron
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

                        // The spritesheet containing all the sprites we calculated in this tileset
                        let sprite_sheet = SpriteSheet {
                            texture: self.texture_handle.clone(),
                            sprites: tile_sprites,
                        };

                        // Insert the sprite sheet, which consists of all the tile sprites,
                        // into world resources for later use
                        MapData {
                            tile_data,
                            sprite_sheet_handle: loader.load_from_data(
                                sprite_sheet,
                                (),
                                &sprite_sheet_storage,
                            ),
                        }
                    })
                    .collect();

                // Now that all the tile sprites/textures are loaded in
                // we can start drawing the tiles for our viewing pleasure
                // Loop over every layer. Because the first layer should be
                // last on the Z axis, we build in reverse.
                for (i, layer) in map.layers.clone().iter().rev().enumerate() {
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
                                sprite_sheet: map_data[0].sprite_sheet_handle.clone(),
                                sprite_number: tile as usize,
                            };

                            // Where we should draw the tile?
                            let tile_transform = {
                                // Bottom Left is 0,0 so we flip it to Top Left with the
                                // ScreenDimensions.height since tiled coordinates start from top
                                let coordinates = (
                                    Float::from(
                                        i_column as f32 * map_data[0].tile_data.tile_size.x as f32,
                                    ),
                                    Float::from(
                                        1f32 - (i_row as f32
                                            * map_data[0].tile_data.tile_size.y as f32),
                                    ),
                                    // Every layer before the last (remember, this is reverse iterating)
                                    // should be further away. `i` is zero-indexed so we need to add one first.
                                    Float::from(-10.0 * (i + 1) as f32),
                                );
                                // Offset the positions by half the tile size so they're nice and snuggly on the screen
                                // Alternatively could use the Sprite offsets instead: [-32.0, 32.0]. Depends on the use case I guess.
                                let offset = (
                                    Float::from(map_data[0].tile_data.tile_size.x as f32 / 2.0),
                                    Float::from(-map_data[0].tile_data.tile_size.y as f32 / 2.0),
                                );
                                Transform::from(Vector3::new(
                                    offset.0 + coordinates.0,
                                    offset.1 + coordinates.1,
                                    coordinates.2,
                                ))
                            };
                            // Create the tile entity
                            entities
                                .build_entity()
                                .with(tile_transform, &mut transform_storage)
                                .with(tile_renderer, &mut sprite_render_storage)
                                .build();
                        }
                    }
                }
        });
    }
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

struct MapData {
    tile_data: TileData,
    sprite_sheet_handle: SpriteSheetHandle,
}
