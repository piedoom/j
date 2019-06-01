use crate::{
    states::MainGameState,
    util::data::CameraConfig};

use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter, RonFormat},
    prelude::*,
    renderer::{
        formats::texture::ImageFormat,
        Texture,
        sprite::{SpriteSheet, SpriteSheetFormat},
    },
};


use tiled::{Map, TmxFormat};

/// Because our map asset contains the paths to textures, we need two load states: one
/// for loading the map file (this one), and another for loading the textures.
pub struct LoadMapState<'a> {
    /// Tracks loaded assets.
    pub progress_counter: ProgressCounter,
    /// Handle to the map
    pub map_handle: Option<Handle<Map>>,
    pub path: &'a str,
}

impl<'a> SimpleState for LoadMapState<'a> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let loader = &data.world.read_resource::<Loader>();
        let map_handle = loader.load(
            self.path,
            TmxFormat,
            &mut self.progress_counter,
            &data.world.read_resource::<AssetStorage<Map>>(),
        );

        self.map_handle = Some(map_handle);
    }

    fn update(
        &mut self,
        _data: &mut StateData<'_, GameData<'_, '_>>,
    ) -> SimpleTrans {
        if self.progress_counter.is_complete() {
            Trans::Switch(Box::new(LoadDataState {
                map_handle: Some(self.map_handle
                    .take()
                    .expect(
                        "Expected `map_handle` to exist when \
                        `progress_counter` is complete."
                    )),
                ..LoadDataState::default()
            }))
        } else {
            Trans::None
        }
    }
}

pub struct LoadDataState {
    /// Tracks loaded assets.
    pub map_texture_progress: ProgressCounter,
    pub camera_config_progress: ProgressCounter,
    pub player_spritesheet_progress: ProgressCounter,
    /// Handle to the map texture. In the future, we may have multiple.
    pub texture_handle: Option<Handle<Texture>>,
    pub camera_config_handle: Option<Handle<CameraConfig>>,
    pub map_handle: Option<Handle<Map>>,
    pub player_spritesheet_handle: Option<Handle<SpriteSheet>>,
}

impl Default for LoadDataState {
    fn default() -> Self {
        Self {
            map_texture_progress: ProgressCounter::new(),
            camera_config_progress: ProgressCounter::new(),
            player_spritesheet_progress: ProgressCounter::new(),
            texture_handle: None,
            camera_config_handle: None,
            map_handle: None,
            player_spritesheet_handle: None,
        }
    }
}

impl<'a> SimpleState for LoadDataState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let loader = &data.world.read_resource::<Loader>();

        // Get our map, which is already definitely loaded in by the previous load state.
        let map_storage = &data.world.read_resource::<AssetStorage<Map>>();
        let map = &map_storage.get(&self.map_handle.clone().unwrap()).unwrap();

        // Now, get our map texture which is defined in the tilemap file.
        let texture_handle = loader.load(
            // This is defined as a relative path in our TMX. Let's get rid of it.
            // Quick and dirty. Much of this should be refactored.
            &map.tilesets[0].images[0].source.clone()[3..],
            ImageFormat::default(),
            &mut self.map_texture_progress,
            &data.world.read_resource::<AssetStorage<Texture>>(),
        );
        self.texture_handle = Some(texture_handle);

        // Here, we will also get data for a bunch of other on-start stuff, like our camera
        self.camera_config_handle = Some(
            loader.load(
                "config/camera.ron",
                RonFormat,
                &mut self.camera_config_progress,
                &data.world.read_resource::<AssetStorage<CameraConfig>>(),
            )
        );

        // Load our player spritesheet
        self.player_spritesheet_handle = Some({
            let texture = loader.load(
                "textures/chars.png",
                ImageFormat::default(),
                (),
                &data.world.read_resource::<AssetStorage<Texture>>(),
            );
            // create the actual spritesheet
            loader.load(
                "textures/chars.png.ron",
                SpriteSheetFormat(texture),
                &mut self.player_spritesheet_progress,
                &data.world.read_resource::<AssetStorage<SpriteSheet>>(),
            )}
        )
    }

    fn update(
        &mut self,
        _data: &mut StateData<'_, GameData<'_, '_>>,
    ) -> SimpleTrans {
        // check all our progress counters, and only move on once all our states are loaded.
        if self.is_complete() {
            Trans::Switch(Box::new(self.build_main_game_state()))
        } else {
            Trans::None
        }
    }
}

impl LoadDataState {
    /// Because we can have N number of progresses to keep track of, we can separate it out
    /// into its own implementation so we don't mess up our update function.
    fn is_complete(&self) -> bool {
        self.map_texture_progress.is_complete() && 
        self.camera_config_progress.is_complete() &&
        self.player_spritesheet_progress.is_complete()
    }

    fn build_main_game_state(&mut self) -> MainGameState {
        MainGameState {
            map_handle: self.map_handle.take().unwrap(),
            texture_handle: self.texture_handle.take().unwrap(),
            camera_config_handle: self.camera_config_handle.take().unwrap(),
            player_spritesheet_handle: self.player_spritesheet_handle.take().unwrap(),
        }
    }
}