use crate::states::MainGameState;

use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter},
    prelude::*,
    renderer::{
        formats::texture::ImageFormat,
        Texture,
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
            Trans::Switch(Box::new(LoadMapAssetsState {
                map_handle: Some(self.map_handle
                    .take()
                    .expect(
                        "Expected `map_handle` to exist when \
                        `progress_counter` is complete."
                    )),
                texture_handle: None,
                progress_counter: ProgressCounter::new(),
            }))
        } else {
            Trans::None
        }
    }
}

pub struct LoadMapAssetsState {
    /// Tracks loaded assets.
    pub progress_counter: ProgressCounter,
    /// Handle to the map texture. In the future, we may have multiple.
    pub texture_handle: Option<Handle<Texture>>,
    pub map_handle: Option<Handle<Map>>,
}

impl<'a> SimpleState for LoadMapAssetsState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let loader = &data.world.read_resource::<Loader>();
        let map_storage = &data.world.read_resource::<AssetStorage<Map>>();
        let map = &map_storage.get(&self.map_handle.clone().unwrap()).unwrap();
        let texture_handle = loader.load(
            // This is defined as a relative path in our TMX. Let's get rid of it.
            // Quick and dirty. Much of this should be refactored.
            &map.tilesets[0].images[0].source.clone()[3..],
            ImageFormat::default(),
            &mut self.progress_counter,
            &data.world.read_resource::<AssetStorage<Texture>>(),
        );

        self.texture_handle = Some(texture_handle);
    }

    fn update(
        &mut self,
        _data: &mut StateData<'_, GameData<'_, '_>>,
    ) -> SimpleTrans {
        if self.progress_counter.is_complete() {
            Trans::Switch(Box::new(MainGameState {
                map_handle: self.map_handle
                    .take()
                    .expect(
                        "Expected `map_handle` to exist when \
                        `progress_counter` is complete."
                    ),
                texture_handle: self.texture_handle
                    .take()
                    .expect(
                        "Expected `texture_handle` to exist when \
                        `progress_counter` is complete."
                    ),
            }))
        } else {
            Trans::None
        }
    }
}
